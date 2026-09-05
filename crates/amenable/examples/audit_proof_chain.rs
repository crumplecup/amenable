//! Look up a registered evidence chain and print its audit report.
//!
//! `char`'s evidence is proven by all three verifier backends. Run with
//! `--features creusot,verus` to see the full chain — one claim, a Kani
//! harness, a Creusot contract, and a Verus postcondition, each printed
//! with its real source:
//!
//!     cargo run --example audit_proof_chain --features creusot,verus
//!
//! With default features only Kani's proof is linked in and printed
//! (Creusot is still an experimental, partial backend, so it's opt-in):
//!
//!     cargo run --example audit_proof_chain

fn main() -> Result<(), amenable::ChainError> {
    let report = amenable::proof_chain("RustStdStandard<char>")?;
    println!("{report}");
    Ok(())
}
