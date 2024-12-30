use crate::prelude::*;
use futures::FutureExt;
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;
use test::TestDesc;
use test::TestDescAndFn;

pub fn run_test(test: &TestDescAndFn) -> TestOutput {
	SweetTestCollector::with_scope(&test.desc, || {
		let func = TestDescAndFnExt::func(test);
		let panic_result =
			std::panic::catch_unwind(|| TestDescExt::result_to_panic(func()));
		match panic_result {
			Ok(()) => Ok(()),
			Err(panic) => {
				Err(TestDescExt::panic_full_format(&test.desc, panic))
			}
		}
	})
}


pub async fn run_test_async(
	(desc, fut): (TestDesc, Pin<Box<dyn Future<Output = ()> + UnwindSafe>>),
) -> (TestDesc, TestOutput) {
	let raw_output = fut
		.catch_unwind()
		.await
		.map_err(|panic| TestDescExt::panic_full_format(&desc, panic));

	(desc, TestOutput::from_result(raw_output))
}
