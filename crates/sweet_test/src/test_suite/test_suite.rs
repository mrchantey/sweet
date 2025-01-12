// use crate::prelude::*;
// use std::collections::HashMap;
// use test::TestDescAndFn;


// /// In sweet, a test suite is all tests in a file.
// #[derive(Debug)]
// pub struct TestSuite {
// 	pub source_file: &'static str,
// 	pub tests: Vec<TestDescAndFn>,
// }

// impl PartialEq for TestSuite {
// 	fn eq(&self, other: &Self) -> bool { self.source_file == other.source_file }
// }

// impl Eq for TestSuite {}

// impl PartialOrd for TestSuite {
// 	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
// 		Some(self.source_file.cmp(other.source_file))
// 	}
// }

// impl Ord for TestSuite {
// 	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
// 		self.source_file.cmp(other.source_file)
// 	}
// }


// impl TestSuite {
// 	pub fn new(source_file: &'static str) -> Self {
// 		Self {
// 			source_file,
// 			tests: Vec::new(),
// 		}
// 	}

// 	pub fn collect_and_run(
// 		config: &TestRunnerConfig,
// 		tests: &[&TestDescAndFn],
// 		func: impl Clone + Fn(&TestDescAndFn) -> TestOutput,
// 	) -> Vec<SuiteOutput> {
// 		Self::collect(tests)
// 			.into_iter()
// 			.map(|suite| suite.run(config, func.clone()))
// 			.collect()
// 	}


// 	fn collect(tests: &[&TestDescAndFn]) -> Vec<Self> {
// 		let mut suites = HashMap::new();
// 		for test in tests.iter() {
// 			let suite = suites
// 				.entry(test.desc.source_file)
// 				.or_insert_with(|| TestSuite::new(test.desc.source_file));
// 			suite.tests.push(TestDescAndFnExt::clone(test));
// 		}
// 		let mut suites: Vec<Self> =
// 			suites.into_iter().map(|(_, suite)| suite).collect();

// 		// sort test cases
// 		for suite in suites.iter_mut() {
// 			suite
// 				.tests
// 				.sort_by(|a, b| a.desc.start_line.cmp(&b.desc.start_line));
// 		}

// 		// sort test suites
// 		suites.sort();
// 		suites
// 	}


// 	pub fn run(
// 		self,
// 		config: &TestRunnerConfig,
// 		run_test: impl Fn(&TestDescAndFn) -> TestOutput,
// 	) -> SuiteOutput {
// 		let mut num_ignored = 0;
// 		let mut num_ran = 0;
// 		let test_outputs = self
// 			.tests
// 			.into_iter()
// 			.map(|test| {
// 				// TODO break this logic out so native can parallel
// 				if test.desc.ignore
// 					|| !TestDescExt::passes_filter(&test.desc, config)
// 				{
// 					num_ignored += 1;
// 					return (test.desc, TestOutput::Ignore);
// 				}
// 				num_ran += 1;
// 				let raw_result = run_test(&test);
// 				(test.desc, raw_result)
// 			})
// 			.collect::<Vec<_>>();

// 		let suite_output = SuiteOutput {
// 			file: self.source_file.to_string(),
// 			test_outputs,
// 		};
// 		suite_output
// 	}
// }
