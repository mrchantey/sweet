#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[sweet::test]
#[rustfmt::skip]
#[should_panic]
fn fails() {
	// let fails = get_fails();
	sweet::prelude::expect(true).to_be_false().unwrap();
}
