#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::libtest_runner::libtest_runner))]

fn main() {}

#[test]
fn pass() {
	sweet::wasm::runner::log_web("ita runninga");
	assert!(false, "aww nuts 😿");
}
