use crate::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
// use web_sys::window;
// use test::*;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	let config = TestRunnerConfig::from_deno_args().unwrap();
	let logger = RunnerLoggerWasm::start(&config);

	std::panic::set_hook(Box::new(PanicStore::panic_hook));

	let suite_results = LibtestSuite::collect_and_run(&config, tests, run_test);

	PartialResultStore {
		config,
		logger,
		suite_results,
	}
	.set();
}


/// Pending async functions cannot be collected in the first initial run
#[wasm_bindgen]
pub async fn run_with_pending() -> Result<(), JsValue> {
	let PartialResultStore {
		config,
		logger,
		mut suite_results,
	} = PartialResultStore::get().map_err(anyhow_to_jsvalue)?;



	let futs =
		SweetTestCollector::drain()
			.into_iter()
			.map(|(desc, fut)| async {
				let raw_result = TestFuture::new(async move {
					fut.await.unwrap_libtest_err();
					Ok(JsValue::NULL)
				})
				.await;

				let failure = TestDescExt::parse_result(&desc, raw_result)
					.err()
					.map(|err| TestDescExt::best_effort_full_err(&desc, &err));

				(desc, failure)
			});
	let sweet_test_results = futures::future::join_all(futs).await;

	SuiteResult::append_sweet_tests(&mut suite_results, sweet_test_results);

	TestRunnerResult::from_suite_results(suite_results).end(&config, logger);

	Ok(())
}



// SET_HOOK.call_once(|| {
// 	std::panic::set_hook(Box::new(|panic_info| {
// 			panic_handling(panic_info.to_string());
// 	}));
// });
// #[cfg(all(
// 	not(feature = "std"),
// 	target_arch = "wasm32",
// 	any(target_os = "unknown", target_os = "none")
// ))]
