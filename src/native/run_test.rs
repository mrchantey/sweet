use crate::prelude::*;
use test::TestDescAndFn;

pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	let func = match test.testfn {
		test::TestFn::StaticTestFn(func) => func,
		_ => panic!("non-static tests are not supported"),
	};

	let result = SweetTestCollector::with_scope(&test.desc, || {
		std::panic::catch_unwind(|| TestDescExt::result_to_panic(func()))
	});

	match result {
		Ok(()) => Ok(()),
		Err(err) => Err(TestDescExt::format_panic(&test.desc, err)),
	}
}
