use super::panic_err_to_string;
use crate::libtest::libtest_result_to_panic;
use crate::libtest::libtesttest_error_location;
use crate::prelude::test_err_full_format;
use test::TestDescAndFn;





pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	match test.testfn {
		test::TestFn::StaticTestFn(func) => {
			let result =
				std::panic::catch_unwind(|| libtest_result_to_panic(func()));
			match result {
				Ok(()) => Ok(()),
				Err(err) => {
					let err = panic_err_to_string(err);
					let loc = libtesttest_error_location(test);
					let full_err = test_err_full_format(&loc, &err, "");
					// let full_err = append
					Err(full_err)
				}
			}
		}
		_ => panic!("non-static tests are not supported"),
	}
}
