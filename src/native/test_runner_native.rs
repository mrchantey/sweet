use super::*;
use crate::test_runner::*;
use crate::test_suite::*;
use anyhow::Result;
use forky_fs::*;
use futures::future::join_all;

pub struct TestRunnerNative;


impl TestRunnerNative {
	#[tokio::main]
	pub async fn run(config: &mut TestRunnerConfig) -> Result<()> {
		// dont exit program on panic?
		// let _ = std::panic::take_hook();
		if config.watch {
			terminal::clear()
		}
		let logger = if config.silent {
			None
		} else {
			Some(RunnerLoggerNative::start(&config))
		};

		let collector = TestCollectorNative::new();

		let to_run = collector.suites_to_run(&config);

		let results = if config.parallel {
			run_group_parallel(to_run, &config).await
		} else {
			TestRunner::run_group_series::<SuiteLoggerNative, TestCaseNative>(
				to_run, &config,
			)
			.await
		};

		if let Some(logger) = logger {
			logger.end(&results);
		}

		let no_tests = results.cases.tests == 0;
		if config.watch || no_tests {
			return Ok(());
		}
		terminal::show_cursor();

		match results.suites.failed {
			0 => Ok(()),
			1 => Err(anyhow::anyhow!(
				"{} test suite failed",
				results.suites.failed
			)),
			_ => Err(anyhow::anyhow!(
				"{} test suites failed",
				results.suites.failed
			)),
		}
	}
}
async fn run_group_parallel(
	to_run: Vec<&TestSuiteNative>,
	config: &TestRunnerConfig,
) -> TestRunnerResult {
	let handles_parallel = to_run.into_iter().map(|suite| {
		let suite = (*suite).clone();
		let config = (*config).clone();
		tokio::spawn(async move {
			suite.run::<SuiteLoggerNativeSimple>(&config).await
		})
	});
	let results = join_all(handles_parallel)
		.await
		.into_iter()
		.collect::<Result<Vec<_>, _>>();

	match results {
		Ok(results) => results.into(),
		Err(e) => panic!("Error in parallel test suite\n{:?}", e),
	}
}

/*
Test Suites: 3 skipped, 42 passed, 42 of 45 total
Tests:       9 skipped, 109 passed, 118 total
Snapshots:   1 passed, 1 total
Time:        23.497 s
Ran all test suites.
*/
