#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
#[cfg(kani)]
use crate::FallibleOperationReportsSuccess;
use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;

impl KaniWitness for RustStdStandard<std::sync::RwLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rwlock_allows_concurrent_reads_but_not_a_write".to_owned(),
            VERIFY_RWLOCK_ALLOWS_CONCURRENT_READS_BUT_NOT_A_WRITE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::RwLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::RwLock<i32>>",
        "kani",
        || <RustStdStandard<std::sync::RwLock<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RWLOCK_ALLOWS_CONCURRENT_READS_BUT_NOT_A_WRITE_SRC, {
        /// Unlike `Mutex`, two read guards can be held at once — but a
        /// write is still exclusive against them, the defining
        /// difference between the two lock types.
        #[kani::proof]
        fn verify_rwlock_allows_concurrent_reads_but_not_a_write() {
            let value: i32 = kani::any();
            let lock = std::sync::RwLock::new(value);
            {
                let r1 = lock.read().unwrap();
                let r2 = lock.read().unwrap();
                assert!(DerefReflectsTheStoredValue::ensures((*r1, value)));
                assert!(
                    DerefReflectsTheStoredValue::ensures((*r2, value)),
                    "two read guards can be held concurrently"
                );
                assert!(
                    FallibleOperationReportsFailure::ensures(lock.try_write().is_err()),
                    "a write is rejected while readers are held"
                );
            }
            assert!(
                FallibleOperationReportsSuccess::ensures(lock.try_write().is_ok()),
                "a write succeeds once the readers are dropped"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::RwLockReadGuard<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rwlock_read_guard_derefs_to_the_value".to_owned(),
            VERIFY_RWLOCK_READ_GUARD_DEREFS_TO_THE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::RwLockReadGuard<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::RwLockReadGuard<'static, i32>>",
        "kani",
        || <RustStdStandard<std::sync::RwLockReadGuard<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RWLOCK_READ_GUARD_DEREFS_TO_THE_VALUE_SRC, {
        /// `.read()` derefs to the locked value.
        #[kani::proof]
        fn verify_rwlock_read_guard_derefs_to_the_value() {
            let value: i32 = kani::any();
            let lock = std::sync::RwLock::new(value);
            let guard = lock.read().unwrap();
            assert!(DerefReflectsTheStoredValue::ensures((*guard, value)));
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rwlock_write_guard_writes_through".to_owned(),
            VERIFY_RWLOCK_WRITE_GUARD_WRITES_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>>",
        "kani",
        || <RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RWLOCK_WRITE_GUARD_WRITES_THROUGH_SRC, {
        /// A write through `.write()`'s guard is visible on a later
        /// read, once the write guard is dropped.
        #[kani::proof]
        fn verify_rwlock_write_guard_writes_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let lock = std::sync::RwLock::new(value);
            {
                let mut guard = lock.write().unwrap();
                assert!(DerefReflectsTheStoredValue::ensures((*guard, value)));
                *guard = updated;
            }
            assert!(DerefReflectsTheStoredValue::ensures((*lock.read().unwrap(), updated)));
        }
    }
}
