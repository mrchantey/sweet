//! example configuration for a test, just two lines and you're good to go
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[test]
fn it_succeeds() { assert!(true) }

#[test]
#[should_panic]
fn it_panics() { panic!("foo") }
