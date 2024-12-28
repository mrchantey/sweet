#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

fn main() {}

// #[test]
// fn passes() {}
// #[test]
// fn passes2() {}
#[test]
fn fails() { sweet::prelude::expect(true).to_be_false().unwrap(); }
// #[test]
// fn fails2() { sweet::prelude::expect(true).to_be_false().unwrap(); }
