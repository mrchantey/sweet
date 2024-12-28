use super::run_test::run_test;
use crate::prelude::*;
extern crate test;

/// maybe we can allow test_main_with_filenames() as a feature

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	if USE_DEFAULT_LIBTEST {
		test_main_with_filenames(tests);
	} else {
		let config = TestRunnerConfig::from_env_args().unwrap();
		let logger = RunnerLoggerNative::start(&config);

		// ⚠️ THIS DISABLES INTERNAL PANICS ⚠️
		std::panic::set_hook(Box::new(|_| {}));
		libtest_runner(tests, &config, log_native, logger, run_test);
	}
}


fn log_native(msg: &str) {
	println!("{}", msg);
}
