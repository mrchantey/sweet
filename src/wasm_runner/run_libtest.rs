use crate::prelude::*;
// use test::*;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	// the setting of a hook itsself will panic
	// TODO pass args from node/deno
	let config = match TestRunnerConfig::from_deno_args() {
		Ok(c) => c,
		Err(e) => {
			crate::log!("{:?}", e);
			std::process::exit(1);
		}
	};
	let logger = RunnerLoggerWasm::start(&config);

	// // it seems in wasm we can only set_hook once, otherwise
	std::panic::set_hook(Box::new(global_store_panic_hook));
	libtest_runner(tests, &config, logger, run_test);
}
