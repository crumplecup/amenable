//! `#[evidence]` takes no arguments. `#[evidence(basis = "..")]` is real
//! syntax -- but for the *derive* helper attribute paired with
//! `#[derive(Evidence)]`, a different mechanism that happens to share
//! this name. Writing it on the bare attribute macro by mistake must
//! fail to compile, not silently ignore the arguments.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RootEvidence;

#[amenable_derive::evidence(basis = "Self")]
impl amenable_core::Evidence for RootEvidence {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}

fn main() {}
