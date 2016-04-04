//! Having these as an integration test checks that the visibility of the macros
//! and their dependencies is valid for use by external libraries.

#[macro_use]
// Rename the import to check that the macros don't use the literal crate name
extern crate hamlet as willy;

// I would really like to do:
//
// ```
// #![no_std]
// #![no_implicit_prelude]
// #[macro_use]
// extern crate std as bob;
// ```
//
// but macros are too unhygienic :(

// We don't `use` anything here to check the macros only use fully qualified
// paths.

#[test]
fn empty_attr_set() {
    assert_eq!(&*attr_set!().into_vec(), &[]);
}

#[test]
fn single_attr() {
    assert_eq!(&*attr_set!(id = "foo").into_vec(),
               &[
                   willy::attr::Attribute::new("id", "foo"),
               ]);
}

#[test]
fn multi_attr() {
    assert_eq!(&*attr_set!(id = "foo", class = "bar").into_vec(),
               &[
                   willy::attr::Attribute::new("id", "foo"),
                   willy::attr::Attribute::new("class", "bar"),
               ]);
}
