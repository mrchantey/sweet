use crate::prelude::*;
use forky::prelude::*;
use test::TestDescAndFn;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn run_test(test: &TestDescAndFn) -> TestOutput {
	SweetTestCollector::with_scope(&test.desc, || {
		// TODO maybe we dont need panicstore if run_no_abort returns correct string
		PanicStore::with_scope(|| {
			let func = TestDescAndFnExt::func(test);
			// dont worry if it returned error, we will catch the panic
			run_no_abort(func);
		})
		.map_err(|panic_err| {
			TestDescExt::best_effort_full_err(&test.desc, &panic_err)
		})
	})
}

/// this function may panic. In js that will abort execution
/// so we send it out of the wasm boundary
fn run_no_abort(func: fn() -> Result<(), String>) {
	// fn run_no_abort(func: fn() -> Result<(), String>) -> Result<(), String> {
	let closure = Closure::from_func_no_args(move || {
		TestDescExt::result_to_panic(func());
	});
	let func: &js_sys::Function = closure.as_ref().unchecked_ref();
	func.call0(&JsValue::NULL).ok();
	// match func.call0(&JsValue::NULL) {
	// 	Ok(_) => Ok(()),
	// 	Err(err) => Err(format!("{:?}", err)),
	// }
}