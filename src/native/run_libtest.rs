use super::run_test::run_test;
use crate::prelude::*;
extern crate test;

/// maybe we can allow test_main_with_filenames() as a feature
const USE_DEFAULT_LIBTEST: bool = false;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	if USE_DEFAULT_LIBTEST {
		test_main_with_filenames(tests);
	} else {
		let config = TestRunnerConfig::from_env_args().unwrap();
		let logger = RunnerLoggerNative::start(&config);

		// ⚠️ THIS DISABLES INTERNAL PANICS ⚠️
		std::panic::set_hook(Box::new(|_| {}));
		let suite_results =
			LibtestSuite::collect_and_run(tests, run_test, log_native);

		let _ = std::panic::take_hook();

		let results = TestRunnerResult::from_suite_results(suite_results);


		logger.end(&config, &results);
	}
}


fn log_native(msg: &str) {
	println!("{}", msg);
}

/// Pretty much run libtest as-is but with pretty filenames for unit tests.
fn test_main_with_filenames(tests: &[&test::TestDescAndFn]) {
	let tests = apply_filenames(tests);
	let tests = tests.iter().collect::<Vec<_>>();
	println!("\n🤘 sweet as! 🤘\n");
	test::test_main_static(&tests);
}

fn apply_filenames(tests: &[&test::TestDescAndFn]) -> Vec<test::TestDescAndFn> {
	tests
		.into_iter()
		.map(|test| {
			let mut test = clone_libtest(test);
			test.desc.name = test::DynTestName(format!(
				"{} - {}",
				test.desc.source_file,
				libtest_short_name(&test)
			));
			// test::StaticTestName(test.desc.source_file);
			test
		})
		.collect()
}
