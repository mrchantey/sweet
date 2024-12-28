use crate::prelude::*;
// use test::*;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	// it seems in wasm we can only set_hook once, otherwise
	// the setting of a hook itsself will panic
	std::panic::set_hook(Box::new(global_store_panic_hook));

	// TODO pass args from node/deno
	let config = TestRunnerConfig::default();
	let logger = RunnerLoggerWasm::start(&config);

	let suite_results = LibtestSuite::collect_and_run(tests, run_test, log_web);

	let runner_result = TestRunnerResult::from_suite_results(suite_results);
	logger.end(&config, &runner_result);
}
