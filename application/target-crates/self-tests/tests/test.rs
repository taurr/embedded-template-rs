#![no_std]
#![no_main]

use {{crate_name}} as _; // memory layout + panic handler etc.

// See https://crates.io/crates/defmt-test for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use defmt::{assert, assert_eq};

    #[test]
    fn assert_true() {
        assert!(true)
    }

    #[test]
    fn assert_eq() {
        assert_eq!(24, 42, "TODO: write actual tests")
    }
}
