//! Look up a registered evidence chain and print its audit report.
//!
//! `char`'s evidence is proven by all three verifier backends when this
//! crate is built with `--features creusot,verus`; with default features
//! (this example), only Kani's proof is linked in and printed.
//!
//! Run with: `cargo run --example audit_proof_chain`

fn main() -> Result<(), amenable::ChainError> {
    let report = amenable::proof_chain("RustStdStandard<char>")?;
    println!("{report}");
    Ok(())
}
