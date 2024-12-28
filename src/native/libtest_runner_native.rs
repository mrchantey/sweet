use crate::prelude::*;
use test::*;

/// maybe we can allow test_main_with_filenames() as a feature
const USE_DEFAULT_LIBTEST: bool = false;

pub fn libtest_runner_native(tests: &[&test::TestDescAndFn]) {
	if USE_DEFAULT_LIBTEST {
		test_main_with_filenames(tests);
	} else {
		let tests = tests
			.iter()
			.map(|t| into_test_case_native(t))
			.collect::<Vec<_>>();
		let suites = TestCollectorNative::cases_to_suites(tests);
		let mut config = TestRunnerConfig::from_env_args().unwrap();
		if let Err(err) = TestRunnerNative::new(suites).run(&mut config) {
			panic!("{}", err);
		}
	}
}

#[cfg(not(target_arch = "wasm32"))]
fn into_test_case_native(test: &TestDescAndFn) -> TestCaseNative {
	TestCaseNative {
		file: StringOrStaticStr::StaticStr(test.desc.source_file),
		name: StringOrStaticStr::String(
			rust_path_stem(&test.desc.name.to_string()).to_string(),
		),
		func: match test.testfn {
			StaticTestFn(f) => TestCaseNativeFunc::SyncString(f),
			_ => panic!("currently only static tests are supported"),
		},
		config: TestCaseConfig {
			skip: test.desc.ignore,
			only: false,
			context: TestRunEnvironment::Unit,
		},
	}
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
			let mut test = make_owned_test(test);
			test.desc.name = test::DynTestName(format!(
				"{} - {}",
				test.desc.source_file,
				rust_path_stem(&test.desc.name.to_string())
			));
			// test::StaticTestName(test.desc.source_file);
			test
		})
		.collect()
}

fn rust_path_stem(path: &str) -> &str {
	path.split("::").last().unwrap_or(path)
}
