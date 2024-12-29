use crate::prelude::*;
use forky::prelude::*;
use test::TestDescAndFn;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn run_test(test: &TestDescAndFn) -> Result<(), String> {
	AsyncTestDescriptions::store(test);

	let func = match test.testfn {
		test::StaticTestFn(func) => func,
		_ => panic!("currently only static tests are supported"),
	};

	let result = PanicStore::with_scope(|| {
		// dont worry if it returned error, we will catch the panic
		SweetTestCollector::with_scope(&test.desc, || run_no_abort(func)).ok();
	});

	match result {
		None => Ok(()),
		Some(err) => Err(TestDescExt::best_effort_full_err(&test.desc, &err)),
	}
}

/// this function may panic. In js that will abort the program
/// so we send it out of the wasm boundary
fn run_no_abort(func: fn() -> Result<(), String>) -> Result<JsValue, JsValue> {
	let closure = Closure::from_func_no_args(move || {
		TestDescExt::result_to_panic(func());
	});
	let func: &js_sys::Function = closure.as_ref().unchecked_ref();
	func.call0(&JsValue::NULL)
}
