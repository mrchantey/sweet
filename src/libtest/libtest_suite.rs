use super::TestDescExt;
use crate::prelude::SuiteResult;
use crate::prelude::SweetTestCollector;
use crate::prelude::TestRunnerConfig;
use crate::utils::log_val;
use std::collections::HashMap;
use test::TestDescAndFn;
use test::TestFn;


#[derive(Debug)]
pub struct LibtestSuite {
	pub source_file: &'static str,
	pub tests: Vec<TestDescAndFn>,
}

impl PartialEq for LibtestSuite {
	fn eq(&self, other: &Self) -> bool { self.source_file == other.source_file }
}

impl Eq for LibtestSuite {}

impl PartialOrd for LibtestSuite {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.source_file.cmp(other.source_file))
	}
}

impl Ord for LibtestSuite {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.source_file.cmp(other.source_file)
	}
}


impl LibtestSuite {
	pub fn new(source_file: &'static str) -> Self {
		Self {
			source_file,
			tests: Vec::new(),
		}
	}

	pub fn collect_and_run(
		config: &TestRunnerConfig,
		tests: &[&TestDescAndFn],
		func: impl Clone + Fn(&TestDescAndFn) -> Result<(), String>,
		gag_output: bool,
	) -> Vec<SuiteResult> {
		Self::collect(tests)
			.iter()
			.map(|suite| suite.run(config, func.clone(), gag_output))
			.collect()
	}


	fn collect(tests: &[&TestDescAndFn]) -> Vec<Self> {
		let mut suites = HashMap::new();
		for test in tests.iter() {
			let suite = suites
				.entry(test.desc.source_file)
				.or_insert_with(|| LibtestSuite::new(test.desc.source_file));
			suite.tests.push(clone_libtest(test));
		}
		let mut suites: Vec<Self> =
			suites.into_iter().map(|(_, suite)| suite).collect();

		// sort test cases
		for suite in suites.iter_mut() {
			suite
				.tests
				.sort_by(|a, b| a.desc.start_line.cmp(&b.desc.start_line));
		}

		// sort test suites
		suites.sort();
		suites
	}


	pub fn run(
		&self,
		config: &TestRunnerConfig,
		run_test: impl Fn(&TestDescAndFn) -> Result<(), String>,
		// wasm tests should hold their tongue until async tests have also run
		gag_output: bool,
	) -> SuiteResult {
		let mut num_ignored = 0;
		let mut num_ran = 0;
		let failures = self
			.tests
			.iter()
			.filter_map(|test| {
				// TODO break this logic out so native can parallel
				if test.desc.ignore
					|| !TestDescExt::passes_filter(&test.desc, config)
				{
					num_ignored += 1;
					return None;
				}
				num_ran += 1;
				let raw_result = run_test(test);
				TestDescExt::parse_result(&test.desc, raw_result).err()
			})
			.collect::<Vec<_>>();

		let result = SuiteResult::new(
			self.source_file.to_string(),
			num_ran,
			num_ignored,
		)
		.with_failed(failures);
		if !config.silent
			&& !gag_output
			&& !SweetTestCollector::contains_async_test(self.source_file)
		{
			log_val(&result.end_str());
		}
		result
	}
}

/// copied from https://github.com/rust-lang/rust/blob/a25032cf444eeba7652ce5165a2be450430890ba/library/test/src/lib.rs#L223
pub fn clone_libtest(test: &&TestDescAndFn) -> TestDescAndFn {
	match test.testfn {
		TestFn::StaticTestFn(f) => TestDescAndFn {
			testfn: TestFn::StaticTestFn(f),
			desc: test.desc.clone(),
		},
		TestFn::StaticBenchFn(f) => TestDescAndFn {
			testfn: TestFn::StaticBenchFn(f),
			desc: test.desc.clone(),
		},
		_ => panic!("non-static tests are not supported"),
	}
}
