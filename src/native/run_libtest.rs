use super::run_test::run_test;
use crate::prelude::*;
extern crate test;

/// maybe we can allow test_main_with_filenames() as a feature

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	let config = TestRunnerConfig::from_env_args().unwrap();
	let logger = RunnerLoggerNative::start(&config);

	std::panic::set_hook(Box::new(|_| {
		// ⚠️ THIS DISABLES INTERNAL PANICS ⚠️
	}));

	libtest_runner(tests, &config, logger, run_test);
}