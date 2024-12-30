use crate::prelude::*;
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
