extern crate test;
use crate::prelude::*;
use test::*;


#[cfg(target_arch = "wasm32")]
pub fn libtest_runner(tests: &[&test::TestDescAndFn]) {
	use runner::log_web;
	log_web("howdy");
	log_web(&format!("here are the tests: {:?}", tests));
	for test in tests {
		match test.testfn {
			StaticTestFn(f) => {
				log_web("running test");
				match f() {
					Ok(_) => log_web("test passed"),
					Err(e) => log_web(&format!("test failed: {}", e)),
				}
			}
			_ => panic!("currently only static tests are supported"),
		}
	}

	// println!("here are the tests: {:?}", tests);
	// let tests = tests
	// 	.iter()
	// 	.map(|t| into_test_case_wasm(t))
	// 	.collect::<Vec<_>>();
	// let suites = TestCollectorWasm::cases_to_suites(tests);
	// let mut config = TestRunnerConfig::from_env_args().unwrap();
	// if let Err(err) = TestRunnerWasm::new(suites).run(&mut config) {
	// 	eprintln!("{}", err);
	// }
}

const USE_DEFAULT_LIBTEST: bool = false;

/// This function is the `main` for the collect_libtest feature.
/// See `libtest_runner` example for how to attach.
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn libtest_runner(tests: &[&test::TestDescAndFn]) {
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
			eprintln!("{}", err);
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

fn make_owned_test(test: &&TestDescAndFn) -> TestDescAndFn {
	match test.testfn {
		StaticTestFn(f) => TestDescAndFn {
			testfn: StaticTestFn(f),
			desc: test.desc.clone(),
		},
		StaticBenchFn(f) => TestDescAndFn {
			testfn: StaticBenchFn(f),
			desc: test.desc.clone(),
		},
		_ => panic!("non-static tests passed to test::test_main_static"),
	}
}
