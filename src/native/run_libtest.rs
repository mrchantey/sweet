// use super::run_test::run_test;
use crate::prelude::*;
extern crate test;
use anyhow::Result;
use futures::future::join_all;
use futures::future::try_join_all;
use tokio::task::LocalSet;
/// maybe we can allow test_main_with_filenames() as a feature

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	if let Err(err) = run_libtest_inner(tests) {
		eprintln!("Sweet Internal Error: {}", err);
		std::process::exit(1);
	}
}

fn run_libtest_inner(tests: &[&test::TestDescAndFn]) -> Result<()> {
	let config = TestRunnerConfig::from_env_args();
	let _logger = RunnerLoggerNative::start(&config);
	// we disable the default panic hook
	// std::panic::set_hook(Box::new(|_| {}));

	// TestCaseContext::set_panic_hook();
	// std::panic::set_hook(Box::new(TestCaseContext::panic_hook));

	let (future_tx, future_rx) = flume::unbounded::<TestDescAndFuture>();
	let (result_tx, result_rx) = flume::unbounded::<TestDescAndResult>();

	tokio::runtime::Runtime::new().unwrap().block_on(async {
		let _recv_result = tokio::spawn(async move {
			while let Ok(result) = result_rx.recv_async().await {
				let status = result.result.status_str();
				println!(
					"{} {}{}",
					status,
					result.desc.name,
					result.result.stdout()
				);
			}
		});

		// let future_rx2 = future_rx.clone();
		// let recv_fut = tokio::spawn(async move {
		// 	while let Ok(future) = future_rx2.recv_async().await {
		// 		let a = tokio::spawn(async move {
		// 			let result = (future.fut)().await;
		// 			result_tx.send_async(TestDescAndResult {
		// 				desc: future.desc,
		// 				result:TestResult::Pass,
		// 			})
		// 		});
		// 	}
		// });

		TestRunnerRayon::collect_and_run(&config, future_tx, result_tx, tests)
			.unwrap();

		// Run the local task set.
		let mut futs = Vec::new();
		while let Ok(fut) = future_rx.try_recv() {
			futs.push(fut);
		}


		let _fut_result = LocalSet::new()
			.run_until(async move {
				let futs = futs.into_iter().map(|future| {
					// while let Ok(future) = future_rx.recv_async().await {
					tokio::task::spawn_local(async move {
						println!("running future");
						let result = (future.fut)().await;
						println!("Result {}: {:?}", future.desc.name, result);
					})
				});
				try_join_all(futs).await
			})
			.await
			.unwrap();

		// recv_task.abort();
		// recv_task.await.unwrap();

		// while let Ok(result) = output_rx.recv() {
		// 	// let result = result.into_result();
		// 	println!("Result {}: {}", result.desc.name, result.result);
		// 	// logger.log_test_output(&output);
		// }
		println!("All tests done");
		Ok(())
	})
	// logger.end();
}
