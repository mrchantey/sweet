#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[test]
#[rustfmt::skip]
fn fails() { 
	sweet::prelude::expect(true).to_be_false().unwrap();
}
