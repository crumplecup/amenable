use super::types::{JiffTimeBackend, JiffVerifier, JiffZoned};
use crate::jiff::backend::instant::local_date_time_descriptor_to_jiff_civil_datetime;
use crate::jiff::backend::instant::offset_date_time_descriptor_to_jiff_parts;
use crate::jiff::backend::zone_conversions::jiff_zoned_to_zoned_date_time_descriptor;
use crate::jiff::backend::zone_conversions::named_time_zone_descriptor_to_jiff_time_zone;
use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    AttachNamedZoneEstablished, AttachNamedZoneEstablishedToken, AttachNamedZoneInput,
    AttachNamedZoneNativeInput, AttachNamedZoneOutput, AttachNamedZonePreconditions,
    AttachNamedZonePreconditionsToken, ConfirmNamedZoneRevisionEstablished,
    ConfirmNamedZoneRevisionEstablishedToken, ConfirmNamedZoneRevisionInput,
    ConfirmNamedZoneRevisionNativeOutput, ConfirmNamedZoneRevisionOutput,
    ConfirmNamedZoneRevisionPreconditions, ConfirmNamedZoneRevisionPreconditionsToken,
    ConfirmZoneAuthorityEstablished, ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput,
    ConfirmZoneAuthorityPreconditionsToken, InvalidDescriptorSource,
    LocalTimeZoneResolutionAuthorityDescriptor, NamedTimeZoneDescriptorBuilder,
    NamedTimeZoneIdentityValid, NamedTimeZoneRevisionBundle, ProvenZonedDateTimeCarrier, RawInput,
    ResolveLocalDateTimeEstablished, ResolveLocalDateTimeInput,
    ResolveLocalDateTimeNativeEstablished, ResolveLocalDateTimeNativeInput,
    ResolveLocalDateTimeNativeOutput, ResolveLocalDateTimeOutput,
    ResolveLocalDateTimePreconditionsToken, ResolvedNamedTimeZone, TemporalError,
    TemporalErrorKind, TemporalInputToken, ZoneAmbiguityResolutionDescriptor,
    ZoneGapResolutionDescriptor, ZonedDateTimeSemanticBundle,
};

// ── Real zone-transition-resolution logic (Phase 4b core) ──────────
//
// amenable_time's own `LocalTimeZoneResolutionAuthorityDescriptor`
// splits disambiguation into TWO independent axes — `ambiguity`
// (earlier/later, for a *fold*) and `gap` (forward/backward, for a
// *gap*) — where jiff's own convenience methods (`.compatible()`/
// `.earlier()`/`.later()`) apply the SAME direction to both cases at
// once. Matching directly on `jiff::tz::AmbiguousOffset`'s own real
// variants (confirmed via source read) is required to honor the two
// axes independently; a first design draft assumed the two cases
// shared the same "earlier means before-field" convention and got it
// wrong — jiff's own real `earlier()`/`later()` doc comments (and their
// `match` bodies) confirm the GAP case is inverted relative to naive
// expectation: "earlier" (the earlier resulting INSTANT) uses the
// `after` offset for a gap (shifting the skipped local time forward,
// which produces an earlier real instant than the naive `before`
// offset would), and "later" uses `before`. For a fold, the mapping is
// the direct one: "earlier" uses `before`, "later" uses `after`.
// amenable_time's own `ShiftForward`/`ShiftBackward` naming names the
// LOCAL-TIME-axis direction directly (forward past the gap = `after`,
// backward before it = `before`), so no inversion is needed there.

/// Resolve a real `jiff::tz::AmbiguousOffset` against amenable_time's
/// own two-axis resolution authority.
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(ambiguous, authority))
)]
fn resolve_ambiguous_offset(
    ambiguous: jiff::tz::AmbiguousOffset,
    authority: &LocalTimeZoneResolutionAuthorityDescriptor,
) -> jiff::tz::Offset {
    match ambiguous {
        jiff::tz::AmbiguousOffset::Unambiguous { offset } => offset,
        jiff::tz::AmbiguousOffset::Gap { before, after } => match authority.gap() {
            ZoneGapResolutionDescriptor::ShiftForward => after,
            ZoneGapResolutionDescriptor::ShiftBackward => before,
        },
        jiff::tz::AmbiguousOffset::Fold { before, after } => match authority.ambiguity() {
            ZoneAmbiguityResolutionDescriptor::PreferEarlier => before,
            ZoneAmbiguityResolutionDescriptor::PreferLater => after,
        },
    }
}

/// Check whether a given offset is one jiff's real zone rules could
/// actually produce for the given local wall-clock time — i.e. it's
/// either the unambiguous offset, or one of the real gap/fold
/// candidates.
///
/// A real, previously-missing check: Phase 4's own `zoned_date_time_
/// descriptor_to_jiff_zoned` (the `TemporalZoneNativeBridge` realize
/// body) never validated this at all — it pinned the instant via
/// `TimeZone::fixed(offset)`, which accepts ANY offset value
/// unconditionally, silently producing a `Zoned` whose local wall-clock
/// reading (recomputed from the real named zone's own rules at the
/// resulting instant) can differ from what was actually requested when
/// the given offset was wrong. `attach_named_zone`/`attach_named_zone_
/// native` below are the edges that are actually supposed to prove
/// `OffsetConsistentWithNamedZone`, so this check belongs here, for
/// real, rather than staying silently absent.
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(local, tz, offset))
)]
pub(super) fn offset_is_consistent_with_named_zone(
    local: jiff::civil::DateTime,
    tz: &jiff::tz::TimeZone,
    offset: jiff::tz::Offset,
) -> bool {
    match tz.to_ambiguous_zoned(local).offset() {
        jiff::tz::AmbiguousOffset::Unambiguous { offset: real } => offset == real,
        jiff::tz::AmbiguousOffset::Gap { before, after }
        | jiff::tz::AmbiguousOffset::Fold { before, after } => offset == before || offset == after,
    }
}

/// Resolve a local wall-clock date-time against a real named zone,
/// applying amenable_time's own resolution authority to any genuine
/// gap/fold ambiguity.
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(local, tz, authority))
)]
fn resolve_local_date_time_to_jiff_zoned(
    local: jiff::civil::DateTime,
    tz: jiff::tz::TimeZone,
    authority: &LocalTimeZoneResolutionAuthorityDescriptor,
) -> Result<jiff::Zoned, TemporalError> {
    let ambiguous = tz.to_ambiguous_zoned(local).offset();
    let offset = resolve_ambiguous_offset(ambiguous, authority);
    let timestamp = offset.to_timestamp(local).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "resolved local date-time out of jiff's representable instant range: {err}"
            )),
        ))
    })?;
    Ok(timestamp.to_zoned(tz))
}

/// Attach a real named zone to an explicit offset date-time, after
/// genuinely checking the given offset is consistent with what the
/// zone's own real rules say for that local time.
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(local, offset, tz))
)]
fn attach_named_zone_to_jiff_zoned(
    local: jiff::civil::DateTime,
    offset: jiff::tz::Offset,
    tz: jiff::tz::TimeZone,
) -> Result<jiff::Zoned, TemporalError> {
    if !offset_is_consistent_with_named_zone(local, &tz, offset) {
        return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "offset {offset:?} is not one of the real offsets {}'s own rules produce for local \
             time {local}",
                tz.iana_name().unwrap_or("<unnamed>"),
            )),
        )));
    }
    let timestamp = offset.to_timestamp(local).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "could not pin the offset date-time to a fixed instant: {err}"
            )),
        ))
    })?;
    Ok(timestamp.to_zoned(tz))
}

// ── Zone factory (descriptor-level) ─────────────────────────────────
//
// `TemporalZoneFactory<JiffVerifier>`'s 5 edges. Unlike
// `TemporalZoneNativeBridge`, these operate on neutral descriptors
// throughout — `resolve_named_zone`'s raw-text parse, plus 4
// descriptor-to-descriptor computations reusing the core logic above
// and Phase 2/4's own conversion helpers.

impl Exchange<RawInput, ResolvedNamedTimeZone, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ResolvedNamedTimeZone, TemporalError> {
        let identifier = input.as_str();
        jiff::tz::TimeZone::get(identifier).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not resolve IANA time zone {identifier:?}: {err}"
                )),
            ))
        })?;
        let descriptor = NamedTimeZoneDescriptorBuilder::default()
            .identifier(identifier)
            .build()
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                    InvalidDescriptorSource::new(format!(
                        "could not build a named time zone descriptor: {err}"
                    )),
                ))
            })?;
        let input_token = <RawInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token =
            <NamedTimeZoneIdentityValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
                input_token,
            );
        Ok(ResolvedNamedTimeZone::new(descriptor, token))
    }
}

impl Exchange<ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ConfirmZoneAuthorityInput,
    ) -> Result<ConfirmZoneAuthorityOutput, TemporalError> {
        // jiff's own Disambiguation semantics apply uniformly to any
        // real named zone -- no zone-specific restriction on which
        // resolution authority is valid -- so the real check here is
        // exactly the zone identity itself.
        let _tz = named_time_zone_descriptor_to_jiff_time_zone(input.request().zone())?;
        let input_token = <ConfirmZoneAuthorityInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <ConfirmZoneAuthorityEstablished as Establish<
            ConfirmZoneAuthorityPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ConfirmZoneAuthorityOutput::new(
            ConfirmZoneAuthorityEstablished::default(),
            token,
        ))
    }
}

impl Exchange<ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ResolveLocalDateTimeInput,
    ) -> Result<ResolveLocalDateTimeOutput, TemporalError> {
        let request = input.request();
        let local = local_date_time_descriptor_to_jiff_civil_datetime(request.timestamp())?;
        let tz = named_time_zone_descriptor_to_jiff_time_zone(request.zone())?;
        let zoned =
            resolve_local_date_time_to_jiff_zoned(local, tz, request.resolution_authority())?;
        let descriptor = jiff_zoned_to_zoned_date_time_descriptor(&zoned)?;
        let input_token = <ResolveLocalDateTimeInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <ResolveLocalDateTimeEstablished as Establish<
            ResolveLocalDateTimePreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ResolveLocalDateTimeOutput::new(descriptor, token))
    }
}

impl Exchange<AttachNamedZoneInput, AttachNamedZoneOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AttachNamedZoneInput,
    ) -> Result<AttachNamedZoneOutput, TemporalError> {
        let request = input.request();
        let (local, offset) = offset_date_time_descriptor_to_jiff_parts(request.timestamp())?;
        let tz = named_time_zone_descriptor_to_jiff_time_zone(request.zone())?;
        let zoned = attach_named_zone_to_jiff_zoned(local, offset, tz)?;
        let descriptor = jiff_zoned_to_zoned_date_time_descriptor(&zoned)?;
        let input_token = <AttachNamedZoneInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <AttachNamedZoneEstablished as Establish<
            AttachNamedZonePreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(AttachNamedZoneOutput::new(descriptor, token))
    }
}

impl Exchange<ConfirmNamedZoneRevisionInput, ConfirmNamedZoneRevisionOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ConfirmNamedZoneRevisionInput,
    ) -> Result<ConfirmNamedZoneRevisionOutput, TemporalError> {
        // Real, honest scoping: jiff exposes no tzdb-revision concept at
        // all (Phase 4's own finding), so "tracks the tzdb revision" is
        // trivially satisfied for this backend -- there is only ever
        // one revision, whichever the running process is linked
        // against. What IS real and worth checking here: that the
        // descriptor's own offset is STILL consistent with the real
        // zone's rules -- the same check `attach_named_zone` performs,
        // re-run rather than assumed to still hold.
        let descriptor = input.request().timestamp();
        let (local, offset) = offset_date_time_descriptor_to_jiff_parts(descriptor.timestamp())?;
        let tz = named_time_zone_descriptor_to_jiff_time_zone(descriptor.zone())?;
        if !offset_is_consistent_with_named_zone(local, &tz, offset) {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "offset {offset:?} is no longer consistent with {}'s real rules for local time \
                 {local}",
                    tz.iana_name().unwrap_or("<unnamed>"),
                )),
            )));
        }
        let input_token = <ConfirmNamedZoneRevisionInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <ConfirmNamedZoneRevisionEstablished as Establish<
            ConfirmNamedZoneRevisionPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ConfirmNamedZoneRevisionOutput::new(
            ConfirmNamedZoneRevisionEstablished::default(),
            token,
        ))
    }
}

// ── Zone factory (native-level) ─────────────────────────────────────
//
// `TemporalNativeZoneFactory<JiffVerifier>`'s 3 edges. These operate
// directly on already-native jiff values (no descriptor conversion at
// all), reusing the exact same core resolution/consistency logic above.

impl
    Exchange<
        ResolveLocalDateTimeNativeInput<JiffTimeBackend>,
        ResolveLocalDateTimeNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ResolveLocalDateTimeNativeInput<JiffTimeBackend>,
    ) -> Result<ResolveLocalDateTimeNativeOutput<JiffTimeBackend>, TemporalError> {
        let request = input.request();
        let zoned = resolve_local_date_time_to_jiff_zoned(
            **request.timestamp(),
            (**request.zone()).clone(),
            request.resolution_authority(),
        )?;
        let input_token = <ResolveLocalDateTimeNativeInput<JiffTimeBackend> as Sidecar<
            JiffVerifier,
        >>::sidecar(&input);
        let token = <ResolveLocalDateTimeNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ResolveLocalDateTimeNativeOutput::new(
            JiffZoned::new(zoned),
            token,
        ))
    }
}

impl
    Exchange<
        AttachNamedZoneNativeInput<JiffTimeBackend>,
        ProvenZonedDateTimeCarrier<JiffZoned>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AttachNamedZoneNativeInput<JiffTimeBackend>,
    ) -> Result<ProvenZonedDateTimeCarrier<JiffZoned>, TemporalError> {
        let request = input.request();
        let zoned = attach_named_zone_to_jiff_zoned(
            *request.timestamp().local(),
            *request.timestamp().offset(),
            (**request.zone()).clone(),
        )?;
        // Unlike the other native-factory edge above, this edge's own
        // output shares `ZonedDateTimeSemanticBundleToken` with the
        // descriptor-level `attach_named_zone` and Phase 4's own
        // TemporalZoneNativeBridge -- there is exactly one real
        // Establish chain for that proposition (`AttachNamedZonePreconditions`
        // -> `AttachNamedZoneEstablished` -> `ZonedDateTimeSemanticBundle`),
        // so this edge walks the SAME chain internally, starting from
        // its own bare `TemporalInputToken` (the native factory's
        // deliberately minimal precondition -- the real work is this
        // exchange body's own runtime check above, not a richer
        // precondition chain).
        let input_token =
            <AttachNamedZoneNativeInput<JiffTimeBackend> as Sidecar<JiffVerifier>>::sidecar(&input);
        let preconditions_token = <AttachNamedZonePreconditions as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(input_token);
        let established_token = <AttachNamedZoneEstablished as Establish<
            AttachNamedZonePreconditionsToken,
            JiffVerifier,
        >>::establish(preconditions_token);
        let bundle_token = <ZonedDateTimeSemanticBundle as Establish<
            AttachNamedZoneEstablishedToken,
            JiffVerifier,
        >>::establish(established_token);
        Ok(ProvenZonedDateTimeCarrier::<JiffZoned>::new(
            JiffZoned::new(zoned),
            bundle_token,
        ))
    }
}

impl
    Exchange<
        ProvenZonedDateTimeCarrier<JiffZoned>,
        ConfirmNamedZoneRevisionNativeOutput,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<JiffZoned>,
    ) -> Result<ConfirmNamedZoneRevisionNativeOutput, TemporalError> {
        let zoned = input.carrier();
        let tz = zoned.time_zone();
        let local = zoned.datetime();
        let offset = zoned.offset();
        if !offset_is_consistent_with_named_zone(local, tz, offset) {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "offset {offset:?} is no longer consistent with {}'s real rules for local time \
                 {local}",
                    tz.iana_name().unwrap_or("<unnamed>"),
                )),
            )));
        }
        // This edge's own input is an already-proven native carrier,
        // not a fresh `TemporalInputToken`-backed request -- there is
        // no existing token on `input` that starts the
        // `ConfirmNamedZoneRevisionPreconditions` chain (its own
        // credential is `TemporalInputToken`, a different proposition
        // than `ZonedDateTimeSemanticBundleToken`), so a fresh root
        // credential is synthesized here, the same way every test
        // helper in this backend's own test suite starts an Establish
        // chain from `TemporalInputToken::new()`.
        let preconditions_token = <ConfirmNamedZoneRevisionPreconditions as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        let established_token = <ConfirmNamedZoneRevisionEstablished as Establish<
            ConfirmNamedZoneRevisionPreconditionsToken,
            JiffVerifier,
        >>::establish(preconditions_token);
        let bundle_token = <NamedTimeZoneRevisionBundle as Establish<
            ConfirmNamedZoneRevisionEstablishedToken,
            JiffVerifier,
        >>::establish(established_token);
        Ok(ConfirmNamedZoneRevisionNativeOutput::new(
            NamedTimeZoneRevisionBundle::default(),
            bundle_token,
        ))
    }
}
