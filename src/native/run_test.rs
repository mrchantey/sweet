use crate::prelude::*;
use test::TestDescAndFn;

pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	let func = match test.testfn {
		test::TestFn::StaticTestFn(func) => func,
		_ => panic!("non-static tests are not supported"),
	};

	let mut result = None;

	SweetTestCollector::with_scope(&test.desc, || {
		result = Some(std::panic::catch_unwind(|| {
			TestDescExt::result_to_panic(func())
		}));
	});

	match result.expect("result must be some") {
		Ok(()) => Ok(()),
		Err(err) => Err(TestDescExt::format_panic(&test.desc, err)),
	}
}
