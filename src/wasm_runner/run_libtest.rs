use crate::prelude::*;
// use test::*;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	std::panic::set_hook(Box::new(global_store_panic_hook));

	// TODO pass args from node/deno
	let config = TestRunnerConfig::default();

	let logger = if config.silent {
		None
	} else {
		Some(RunnerLoggerWasm::start(&config))
	};


	let suite_results = LibtestSuite::collect(tests)
		.iter()
		.map(run_suite)
		.collect::<Vec<_>>();

	let runner_result = TestRunnerResult::from_suite_results(suite_results);

	if let Some(logger) = logger {
		logger.end(&runner_result);
	}
}
