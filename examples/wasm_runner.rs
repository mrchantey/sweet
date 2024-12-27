#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::libtest_runner::libtest_runner))]

// use wasm_bindgen_test::*;

// use sweet::wasm::runner::log_web;

use sweet::wasm::runner::log_web;

// use wasm_bindgen_test::*;
pub fn main() {
	// js_sys:
	// log_web("hello world!");

	// #[cfg(test)]
	// log_web("its a test");
	// println!("hello world!");
}



#[test]
fn pass() -> anyhow::Result<()> {
	log_web("ita runninga");

	panic!("ita faila");
	// anyhow::bail!("ita faila");
	// Err("ita faila".to_string())
}
// #[wasm_bindgen_test::wasm_bindgen_test]
// fn pass() {
// 	assert_eq!(1, 1);
// }
// #[cfg(test)]
// mod tests {
// 	extern crate test;

// 	fn libtest_runner(tests: &[&test::TestDescAndFn]) {
// 		println!("here are the tests: {:?}", tests);
// 		// let tests = tests
// 		// 	.iter()
// 		// 	.map(|t| into_test_case_wasm(t))
// 		// 	.collect::<Vec<_>>();
// 		// let suites = TestCollectorWasm::cases_to_suites(tests);
// 		// let mut config = TestRunnerConfig::from_env_args().unwrap();
// 		// if let Err(err) = TestRunnerWasm::new(suites).run(&mut config) {
// 		// 	eprintln!("{}", err);
// 		// }
// 	}
// }
