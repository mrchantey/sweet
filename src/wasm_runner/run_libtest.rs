use crate::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
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

	let promises = AsyncTestPromises::collect().await?;

	crate::log!("🚀🚀🚀");

	Ok(())
}

fn anyhow_to_jsvalue(e: anyhow::Error) -> JsValue {
	JsValue::from_str(&format!("{:?}", e))
}
