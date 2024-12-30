use crate::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
// use web_sys::window;
// use test::*;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	let config = TestRunnerConfig::from_deno_args().unwrap();
	let logger = RunnerLoggerWasm::start(&config);

	std::panic::set_hook(Box::new(PanicStore::panic_hook));

	let suite_outputs = TestSuite::collect_and_run(&config, tests, run_test);

	let (async_suite_outputs, suite_results) =
		SuiteOutput::finalize_sync(&config, suite_outputs);

	PartialResultStore {
		config,
		logger,
		suite_results,
		async_suite_outputs,
	}
	.set();
}


/// Pending async functions cannot be collected in the first initial run
#[wasm_bindgen]
pub async fn run_with_pending() -> Result<(), JsValue> {
	let PartialResultStore {
		config,
		logger,
		suite_results,
		async_suite_outputs,
	} = PartialResultStore::take();

	let futs =
		SweetTestCollector::drain()
			.into_iter()
			.map(|(desc, fut)| async {
				let raw_result = TestFuture::new(async move {
					fut.await;
					Ok(JsValue::NULL)
				})
				.await
				.map_err(|err| TestDescExt::best_effort_full_err(&desc, &err));
				let result = TestOutput::from_result(raw_result);

				(desc, result)
			});
	let async_test_outputs = futures::future::join_all(futs).await;
	TestRunnerResult::finalize(
		config,
		logger,
		suite_results,
		async_suite_outputs,
		async_test_outputs,
	);
	Ok(())
}
