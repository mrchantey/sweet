use crate::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
// use web_sys::window;
// use test::*;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	// TODO pass args from node/deno
	let config = match TestRunnerConfig::from_deno_args() {
		Ok(c) => c,
		Err(e) => {
			crate::log!("{:?}", e);
			std::process::exit(1);
		}
	};
	let logger = RunnerLoggerWasm::start(&config);

	std::panic::set_hook(Box::new(PanicStore::panic_hook));

	let suite_results =
		LibtestSuite::collect_and_run(&config, tests, run_test, true);
	// let _ = std::panic::take_hook();

	PartialResultStore::new(logger, suite_results)
		.set()
		.unwrap();
}


/// Pending async functions cannot be collected in the first initial run
#[wasm_bindgen]
pub async fn run_with_pending() -> Result<(), JsValue> {
	// let PartialResultStore { logger, results } =
	// 	PartialResultStore::get().map_err(anyhow_to_jsvalue)?;

	// window().unwrap();


	AsyncTestPromises::await_and_collect().await?;

	crate::log!("🚀🚀\n");
	// crate::log!("🚀🚀🚀 \n{:?}", panics);

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
