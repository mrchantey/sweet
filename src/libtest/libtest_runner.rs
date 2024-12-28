use crate::prelude::*;
use test::TestDescAndFn;



pub fn libtest_runner(
	tests: &[&test::TestDescAndFn],
	config: &TestRunnerConfig,
	log: impl Clone + Fn(&str),
	logger: impl RunnerLogger,
	run_test: impl Clone + Fn(&TestDescAndFn) -> Result<(), String>,
) {
	if USE_DEFAULT_LIBTEST {
		return test_main_with_filenames(tests);
	}

	let suite_results =
		LibtestSuite::collect_and_run(config, tests, run_test, log);

	let runner_result = TestRunnerResult::from_suite_results(suite_results);
	logger.end(&config, &runner_result);

	if runner_result.did_fail() {
		std::process::exit(1);
	}
}
const USE_DEFAULT_LIBTEST: bool = false;


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
