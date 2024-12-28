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

	pub fn collect(tests: &[&TestDescAndFn]) -> Vec<Self> {
		let mut suites = HashMap::new();
		for test in tests.iter() {
			let suite = suites
				.entry(test.desc.source_file)
				.or_insert_with(|| LibtestSuite::new(test.desc.source_file));
			suite.tests.push(clone_test(test));
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
}

/// copied from https://github.com/rust-lang/rust/blob/a25032cf444eeba7652ce5165a2be450430890ba/library/test/src/lib.rs#L223
fn clone_test(test: &&TestDescAndFn) -> TestDescAndFn {
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
