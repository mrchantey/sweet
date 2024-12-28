use crate::prelude::SuiteResult;
use std::collections::HashMap;
use test::ShouldPanic;
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
		tests: &[&TestDescAndFn],
		func: impl Clone + Fn(&TestDescAndFn) -> Result<(), String>,
		log: impl Clone + Fn(&str),
	) -> Vec<SuiteResult> {
		Self::collect(tests)
			.iter()
			.map(|suite| suite.run(func.clone(), log.clone()))
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
		func: impl Fn(&TestDescAndFn) -> Result<(), String>,
		log: impl Fn(&str),
	) -> SuiteResult {
		let mut num_ignored = 0;
		let mut num_ran = 0;
		let failures = self
			.tests
			.iter()
			.filter_map(|test| {
				if test.desc.ignore {
					num_ignored += 1;
					return None;
				}
				num_ran += 1;
				match (func(test), test.desc.should_panic) {
					(Ok(_), ShouldPanic::No) => None,
					(Ok(_), ShouldPanic::Yes) => {
						Some(format!("Expected panic"))
					}
					(Ok(_), ShouldPanic::YesWithMessage(msg)) => {
						Some(format!("Expected panic: {}", msg))
					}
					(Err(err), ShouldPanic::No) => Some(err),
					(Err(_), ShouldPanic::Yes) => None,
					(Err(_), ShouldPanic::YesWithMessage(_)) => None,
				}
			})
			.collect::<Vec<_>>();

		let result =
			SuiteResult::new(self.source_file.into(), num_ran, num_ignored)
				.with_failed(failures);
		log(&result.end_str());
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
