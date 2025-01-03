// use super::run_test::run_test;
use crate::prelude::*;
use std::sync::Arc;
extern crate test;
use anyhow::Result;
use tokio::task::JoinHandle;
/// maybe we can allow test_main_with_filenames() as a feature

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	if let Err(err) = run_libtest_inner(tests) {
		eprintln!("Sweet Internal Error: {}", err);
		std::process::exit(1);
	}
}

fn run_libtest_inner(tests: &[&test::TestDescAndFn]) -> Result<()> {
	tokio::runtime::Runtime::new()
		.unwrap()
		.block_on(async move {
			let (future_tx, _future_rx) =
				flume::unbounded::<TestDescAndFuture>();
			let (result_tx, result_rx) =
				flume::unbounded::<TestDescAndResult>();
			let config = Arc::new(TestRunnerConfig::from_env_args());
			let mut logger = RunnerLogger::start(config.clone(), tests);
			let recv_result: JoinHandle<Result<RunnerLogger>> =
				tokio::spawn(async move {
					while let Ok(result) = result_rx.recv_async().await {
						logger.on_result(result)?;
					}
					Ok(logger)
				});

			// let logger = RunnerLoggerNative::start(&config);
			TestRunnerRayon::collect_and_run(
				&config, future_tx, result_tx, tests,
			)
			.unwrap();

			let logger = recv_result.await??;

			logger.end();

			Ok(())
		})
	// logger.end();
}
