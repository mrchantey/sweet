use super::run_test::run_test;
use crate::prelude::*;
extern crate test;

/// maybe we can allow test_main_with_filenames() as a feature

pub fn run_libtest(tests: &[&test::TestDescAndFn]) {
	let config = TestRunnerConfig::from_env_args().unwrap();
	let logger = RunnerLoggerNative::start(&config);

	std::panic::set_hook(Box::new(|panic| {
		if false {
			// TODO bactrace
			let payload = panic.payload_as_str().unwrap_or("no payload");
			eprintln!(
				"Uncaught Sweet Panic\nPlease file a bug report\n{:?}",
				payload
			);
		}
	}));

	let suite_outputs = LibtestSuite::collect_and_run(&config, tests, run_test);


	let (async_suite_outputs, suite_results) =
		SuiteOutput::finalize_sync(&config, suite_outputs);

	// begin async shenanigans
	let async_test_outputs =
		tokio::runtime::Runtime::new().unwrap().block_on(async {
			let futs =
				SweetTestCollector::drain().into_iter().map(run_test_async);
			futures::future::join_all(futs).await
		});

	TestRunnerResult::finalize(
		config,
		logger,
		suite_results,
		async_suite_outputs,
		async_test_outputs,
	);
}

// async fn run_native_parallel(
// 	to_run: &TestCaseNativeSplit<'_>,
// ) -> anyhow::Result<Vec<Error>> {
// 	if to_run.series.len() > 0 {
// 		panic!(
// 			"\n\nattempted to run suites containing 'non_send' in parallel\n\n"
// 		)
// 	}


// 	let handles_parallel = to_run
// 		.parallels
// 		.iter()
// 		.map(|(t, f)| {
// 			let t = (*t).clone();
// 			let f = (*f).clone();
// 			tokio::spawn(async move {
// 				let result = unwrap_panic_async(f()).await;
// 				t.format_error(result)
// 			})
// 		})
// 		.collect::<Vec<_>>();

// 	let results_parallel = tokio::spawn(async move {
// 		futures::future::join_all(handles_parallel).await
// 	}); // TODO seems like awkward way to force handles to run

// 	let results_sync_str = to_run
// 		.syncs_str
// 		.par_iter()
// 		.map(|(t, f)| {
// 			let result = unwrap_panic_str(&f);
// 			t.format_error(result)
// 		})
// 		.collect::<Vec<_>>(); //blocks until syncs complete
// 	let results_sync = to_run
// 		.syncs
// 		.par_iter()
// 		.map(|(t, f)| {
// 			let result = unwrap_panic(&f);
// 			t.format_error(result)
// 		})
// 		.collect::<Vec<_>>(); //blocks until syncs complete


// 	// let results_parallel = futures::future::join_all(handles_parallel).await
// 	let results_parallel = results_parallel
// 		.await? //blocks until parallels complete
// 		.into_iter()
// 		.collect::<std::result::Result<Vec<_>, JoinError>>()?;
// 	let errs = results_sync
// 		.into_iter()
// 		.chain(results_sync_str)
// 		.chain(results_parallel)
// 		.filter_map(|r| r.err())
// 		.collect();
// 	Ok(errs)
// }
// async fn run_group_parallel(
// 	to_run: Vec<TestSuiteNative>,
// 	config: &TestRunnerConfig,
// ) -> TestRunnerResult {
// 	let handles_parallel = to_run.into_iter().map(|suite| {
// 		let config = (*config).clone();
// 		tokio::spawn(async move {
// 			suite.run::<SuiteLoggerNativeSimple>(&config).await
// 		})
// 	});
// 	let results = join_all(handles_parallel)
// 		.await
// 		.into_iter()
// 		.collect::<Result<Vec<_>, _>>();

// 	match results {
// 		Ok(results) => results.into(),
// 		Err(e) => panic!("Error in parallel test suite\n{:?}", e),
// 	}
// }
