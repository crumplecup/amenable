mod enum_fixtures;
mod generic_fixtures;
mod nested_fixtures;
pub(crate) mod shared;
mod struct_fixtures;

pub use enum_fixtures::{NamedEnumFixture, TupleEnumFixture, UnitEnumFixture};
pub use generic_fixtures::{
    ConcreteGenericEnumFixture, ConcreteGenericStructFixture, ConcreteGenericTupleStructFixture,
};
pub use nested_fixtures::{NestedStructFixture, NestedTupleStructFixture};
pub(crate) use shared::for_each_fixture_type;
pub use shared::{DeriveFixtureKind, FixtureCase, expected_keys, expected_report, expected_values};
pub use struct_fixtures::{
    CheckedPlusTrivialStructFixture, NamedStructFixture, TupleStructFixture, UnitStructFixture,
};
