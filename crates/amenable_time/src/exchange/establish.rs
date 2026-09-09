//! Output tokens and their [`Establish`](amenable_core::Establish) edges.
//!
//! Every temporal exchange mints its output token the one lawful way:
//! `<Proposition as Establish<TemporalInputToken, V>>::establish(input.
//! sidecar())`. Each output token is a private-field unit struct — a
//! caller cannot forge one — generated together with its `Establish` impl
//! by `#[amenable_derive::establish]`'s verifier-less form: one
//! `impl<V: Verifier> Establish<TemporalInputToken, V> for P where P:
//! Witness<V>` per proposition, gated per-backend by whichever `Witness<
//! V>` proof exists. Exactly the `amenable_gaap::tokens` pattern.
//!
//! Phase 4 Step 1 wires only the `parse_calendar_date` canary; the rest
//! land per method in Step 2+.

/// Lawful token: [`CalendarDateValid`](crate::CalendarDateValid) was
/// established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CalendarDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::CalendarDateValid"
)]
pub struct CalendarDateValidToken(());
