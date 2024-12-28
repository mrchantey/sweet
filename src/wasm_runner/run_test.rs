use crate::prelude::*;
use forky::prelude::*;
use test::TestDescAndFn;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	match test.testfn {
		test::StaticTestFn(f) => {
			let closure = Closure::from_func_no_args(move || {
				libtest_result_to_panic(f());
			});

			let func: &js_sys::Function = closure.as_ref().unchecked_ref();

			let result = func.call0(&JsValue::NULL);

			match result {
				Ok(_) => Ok(()),
				Err(
					_, // the error returned from a panic is just an Unreachable with backtrace
				) => {
					let err = PanicStore::get();
					if AsyncTestDescriptions::try_store(test, &err)
						.expect("TODO HOW TO HANDLE AN ACTUAL ERROR, NESTED?")
					{
						return Ok(());
					}

					let loc = libtesttest_error_location(test);

					// we dont get backtrace in wasm so just point to test start
					let backtrace = test_err_link(
						test.desc.source_file,
						test.desc.start_line,
						test.desc.start_col,
					);

					let full_err = test_err_full_format(&loc, &err, &backtrace);
					// let full_err = append
					Err(full_err)
				}
			}
		}
		_ => panic!("currently only static tests are supported"),
	}
}
