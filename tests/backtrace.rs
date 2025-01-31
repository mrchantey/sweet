#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]


use sweet::prelude::*;


#[test]
#[ignore]
fn fails() {
	// this is correct bactrace in external crates
	expect(true).to_be_false();
	// this is incorrect
	expect(true).to_be(false);
	// absolute mystery
}
