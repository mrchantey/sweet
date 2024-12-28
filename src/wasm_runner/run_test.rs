use crate::prelude::*;
use forky::prelude::*;
use test::TestDescAndFn;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	AsyncTestDescriptions::store(test);

	let result = run_no_abort(test);
	match result {
		Ok(_) => Ok(()),
		Err(
			_, // the error returned from a panic is just an Unreachable with backtrace
		) => {
			// return Ok(());
			// WRONG
			let err = "unreachable panic";
			// let err = PanicStore::get();

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

/// this function may panic. in js that will abort the program
/// so we send it out of the wasm boundary
fn run_no_abort(test: &TestDescAndFn) -> Result<JsValue, JsValue> {
	let func = match test.testfn {
		test::StaticTestFn(func) => func,
		_ => panic!("currently only static tests are supported"),
	};

	let closure = Closure::from_func_no_args(move || {
		libtest_result_to_panic(func());
	});

	let func: &js_sys::Function = closure.as_ref().unchecked_ref();

	func.call0(&JsValue::NULL)
}
