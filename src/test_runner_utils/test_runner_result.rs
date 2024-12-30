use super::*;
use crate::test_suite::*;
use colorize::*;
use std::time::Duration;
use test::TestDesc;

#[derive(Debug)]
pub struct TestRunnerResult {
	pub suite_results: Vec<SuiteResult>,
	pub suites: ResultCount,
	pub cases: ResultCount, //TODO probably newtype this
}

impl Into<TestRunnerResult> for Vec<SuiteResult> {
	fn into(self) -> TestRunnerResult {
		TestRunnerResult::from_suite_results(self)
	}
}

impl TestRunnerResult {
	/// Common finalization for both native and wasm runners
	pub fn finalize(
		config: TestRunnerConfig,
		logger: impl RunnerLogger,
		mut suite_results: Vec<SuiteResult>,
		mut async_suite_outputs: Vec<SuiteOutput>,
		async_test_outputs: Vec<(TestDesc, TestOutput)>,
	) {
		SuiteOutput::extend_test_outputs(
			&mut async_suite_outputs,
			async_test_outputs,
		);
		let async_results =
			SuiteOutput::finalize_all(&config, async_suite_outputs);
		suite_results.extend(async_results);

		Self::from_suite_results(suite_results).end(&config, logger);
	}





	pub fn did_fail(&self) -> bool { self.cases.failed > 0 }

	pub fn from_suite_results(suite_results: Vec<SuiteResult>) -> Self {
		let mut suites = ResultCount::default();
		let cases = suite_results.iter().fold(
			ResultCount::default(),
			|mut acc, item| {
				acc.tests += item.num_tests;
				acc.failed += item.failed.len();
				acc.skipped += item.num_ignored;

				suites.tests += 1;
				if item.failed.len() > 0 {
					suites.failed += 1;
				}

				acc
			},
		);
		TestRunnerResult {
			suite_results,
			suites,
			cases,
		}
	}


	pub fn end_str(&self, duration: Duration) -> String {
		let mut post_run = String::from("\n");

		if self.cases.tests == 0 {
			post_run += "No Tests Found\n".red().as_str();
			return post_run;
		} else if self.cases.failed == 0 {
			post_run +=
				"All tests passed\n".bold().cyan().underlined().as_str();
		}

		post_run += self.suites.pretty_print("Suites:\t\t").as_str();
		post_run.push('\n');
		post_run += self.cases.pretty_print("Tests:\t\t").as_str();
		post_run.push('\n');
		post_run += Self::print_time(duration).as_str();
		post_run
	}

	fn print_time(duration: Duration) -> String {
		let millis = duration.as_millis();
		let prefix = "Time:\t\t".bold();
		if millis < 100 {
			format!("{}{} ms\n\n", prefix, millis)
		} else {
			format!("{}{:.2} s\n\n", prefix, millis as f32 * 0.001)
		}
	}


	pub fn end(&self, config: &TestRunnerConfig, logger: impl RunnerLogger) {
		logger.end(&config, self);
		if !config.watch && self.did_fail() {
			std::process::exit(1);
		}
	}
}
