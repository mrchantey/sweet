use crate::prelude::*;
use test::TestDesc;





/// The output of a suite that has fully ran.
/// This is used for both sync and async tests, so be careful
#[derive(Debug)]
pub struct SuiteOutput {
	pub file: String,
	pub test_outputs: Vec<(TestDesc, TestOutput)>,
}


impl SuiteOutput {
	/// # Panics
	/// if the suite contains async tests
	pub fn finalize_all(
		config: &TestRunnerConfig,
		suites: Vec<Self>,
	) -> Vec<SuiteResult> {
		let (async_suites, results) = Self::finalize_sync(config, suites);
		if !async_suites.is_empty() {
			panic!("suites contain async tests");
		}
		results
	}

	/// The suites that do not contain async tests should
	/// be finalized before awaiting the async tests.
	///
	/// Note any suite that contains at least one async test is an async suite
	pub fn finalize_sync(
		config: &TestRunnerConfig,
		suites: Vec<Self>,
	) -> (Vec<Self>, Vec<SuiteResult>) {
		let (async_suites, sync_suites): (Vec<_>, Vec<_>) =
			suites.into_iter().partition(|suite| suite.contains_async());

		let results = sync_suites
			.into_iter()
			.map(|suite| {
				let result = suite.into_result();
				if !config.silent {
					crate::log!("{}", result.end_str());
				}
				result
			})
			.collect();

		(async_suites, results)
	}

	fn contains_async(&self) -> bool {
		self.test_outputs
			.iter()
			.any(|(_, output)| matches!(output, TestOutput::Async))
	}

	/// Convert into a suite result
	/// # Panics
	/// if the suite contains async tests
	fn into_result(self) -> SuiteResult {
		let num_tests = self.test_outputs.len();
		let mut num_ignored = 0;

		let failed = self
			.test_outputs
			.into_iter()
			.filter_map(|(desc, output)| match output.into_result(&desc) {
				Some(Ok(())) => None,
				None => {
					num_ignored += 1;
					None
				}
				Some(Err(err)) => Some(err),
			})
			.collect::<Vec<_>>();


		SuiteResult {
			file: self.file,
			num_tests,
			num_ignored,
			failed,
		}
	}

	/// Place tests in their suite
	/// # Panics
	/// - if a test does not match any suite
	/// - if the suite does not contain the test
	pub fn extend_test_outputs(
		suite_outputs: &mut Vec<Self>,
		test_outputs: Vec<(TestDesc, TestOutput)>,
	) {
		for test_output in test_outputs {
			let suite_output = suite_outputs
				.iter_mut()
				.find(|suite| suite.file == test_output.0.source_file)
				.unwrap_or_else(|| {
					panic!(
						"test file does not match any suite: {}",
						test_output.0.source_file
					)
				});

			let output = suite_output
				.test_outputs
				.iter_mut()
				.find(|(desc, _)| {
					TestDescExt::is_equal_location(&test_output.0, desc)
				})
				.expect("suite does not contain test");
			output.1 = test_output.1;
		}
	}
}
