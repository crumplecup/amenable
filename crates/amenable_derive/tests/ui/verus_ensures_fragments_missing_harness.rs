//! A harness name with no real carrier file behind it must fail to
//! compile with a real error naming the harness, not silently produce
//! an empty slice or panic at runtime.

fn main() {
    let _: &[&str] = amenable_derive::verus_ensures_fragments!("this_harness_does_not_exist");
}
