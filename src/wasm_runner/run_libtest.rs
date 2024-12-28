use crate::prelude::*;
use forky::prelude::StringExt;
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

	let results = LibtestSuite::collect_and_run(&config, tests, run_test, true);
	// let _ = std::panic::take_hook();

	PartialResultStore {
		config,
		logger,
		results,
	}
	.set()
	.unwrap();
}


/// Pending async functions cannot be collected in the first initial run
#[wasm_bindgen]
pub async fn run_with_pending() -> Result<(), JsValue> {
	let PartialResultStore {
		config,
		logger,
		mut results,
	} = PartialResultStore::get().map_err(anyhow_to_jsvalue)?;

	let async_results = AsyncTestPromises::await_and_collect().await?;

	// TODO we need the successes as well for should_panic
	for (async_desc, async_err) in async_results {
		let suite = results
			.iter_mut()
			.find(|suite| {
				suite.file.to_string_lossy() == async_desc.source_file
			})
			.expect("async suite not found");
		// INCORRECT but we'll sort it out later
		// should be identical to run_test
		suite.failed.push(async_err);
	}

	// crate::log!("🚀🚀🚀 \n{:?}", results);
	if !config.silent {
		let suites_output =
			results.iter().fold(String::new(), |mut acc, result| {
				acc.push_string(&result.end_str());
				acc
			});
		crate::log!("{}", suites_output);
	}

	TestRunnerResult::from_suite_results(results).end(&config, logger);

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
