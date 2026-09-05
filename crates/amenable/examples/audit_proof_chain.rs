//! Look up a registered evidence chain and print its audit report.
//!
//! `char`'s evidence is proven by all three verifier backends. Run with
//! `--features creusot,verus` to see the full chain — one claim, a Kani
//! harness, a Creusot contract, and a Verus postcondition, each printed
//! with its real source:
//!
//!     cargo run --example audit_proof_chain --features creusot,verus
//!
//! All three backends are at feature parity; `creusot`/`verus` are opt-in
//! only to keep the default build's dependency tree light. With default
//! features just Kani's proof is linked in and printed:
//!
//!     cargo run --example audit_proof_chain

fn main() -> Result<(), amenable::ChainError> {
    let report = amenable::proof_chain("RustStdStandard<char>")?;
    println!("{report}");
    Ok(())
}
