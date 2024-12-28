use crate::prelude::*;
use forky::prelude::*;
use std::panic::PanicHookInfo;
use test::TestDescAndFn;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	let testid = TestId::next();

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
					let err = get_stored_panic(testid);
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


const VAR_NAME: &str = "__sweet_panic_output";

/// Panics are stored in the global window object
/// and can be accessed by the test runner
pub fn global_store_panic_hook(info: &PanicHookInfo) {
	let payload = info.payload_as_str().unwrap_or("no panic message");
	let window = web_sys::window().expect("no global window exists");
	js_sys::Reflect::set(&window, &VAR_NAME.into(), &payload.into()).unwrap();
}

/// Collect the message from the panic hook
fn get_stored_panic(_id: TestId) -> String {
	let window = web_sys::window().expect("no global window exists");
	let output = js_sys::Reflect::get(&window, &VAR_NAME.into())
		// js_sys::Reflect::get(&window, &JsValue::from_str(&id.var_name()))
		.unwrap();
	output.as_string().unwrap()
	// "".to_string()
}
