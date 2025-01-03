use crate::prelude::*;
use anyhow::Result;
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub fn run_libtest(tests: &[&test::TestDescAndFn]) -> Result<()> {
	let config = TestRunnerConfig::from_deno_args().unwrap();
	let config = Arc::new(config);
	let mut logger = RunnerLogger::start(config.clone(), tests);


	let (result_tx, result_rx) = flume::unbounded::<TestDescAndResult>();

	let tests = collect_runnable_tests(&config, &result_tx, tests)?;
	flush_rx(&mut logger, &result_rx);

	std::panic::set_hook(Box::new(PanicStore::panic_hook));
	let futures = run_wasm_tests_sync(tests, &result_tx);
	flush_rx(&mut logger, &result_rx);



	PartialRunnerState {
		logger,
		futures,
		result_tx,
		result_rx,
	}
	.set();
	Ok(())
}

fn flush_rx(
	logger: &mut RunnerLogger,
	result_rx: &flume::Receiver<TestDescAndResult>,
) {
	while let Ok(result) = result_rx.try_recv() {
		logger.on_result(result).unwrap();
	}
}


/// Pending async functions cannot be collected in the first initial run
#[wasm_bindgen]
pub async fn run_with_pending() -> Result<(), JsValue> {
	let PartialRunnerState {
		mut logger,
		futures,
		result_tx,
		result_rx,
	} = PartialRunnerState::take().ok_or("no partial runner state")?;

	let futs = futures.into_iter().map(|fut| async {
		TestFuture::new(fut.desc, result_tx.clone(), async move {
			(fut.fut)().await.ok();
			Ok(JsValue::NULL)
		})
		.await;
	});
	let _async_test_outputs = futures::future::join_all(futs).await;

	flush_rx(&mut logger, &result_rx);
	logger.end();
	Ok(())
}