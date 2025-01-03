use crate::prelude::*;
use flume::Sender;
use forky::web::ClosureFnMutT2Ext;
use test::TestDescAndFn;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn run_wasm_tests_sync(
	tests: Vec<TestDescAndFn>,
	result_tx: &Sender<TestDescAndResult>,
) -> Vec<TestDescAndFuture> {
	tests
		.into_iter()
		.filter_map(|test| {
			let func = TestDescAndFnExt::func(&test);

			let result = SweetTestCollector::with_scope(&test.desc, || {
				PanicStore::with_scope(&test.desc, || {
					// dont worry if it returned error, we will catch the panic
					run_no_abort(func)
				})
			});

			match result {
				Ok(PanicStoreOut::Panicked(result)) => {
					result_tx.send(result).expect("channel was dropped");
					None
				}
				Ok(PanicStoreOut::Ok(_)) => {
					let test_result =
						TestResult::from_test_result(Ok(()), &test.desc);
					result_tx
						.send(TestDescAndResult::new(
							test.desc.clone(),
							test_result,
						))
						.expect("channel was dropped");
					None
				}
				Err(val) => Some(val),
			}
		})
		.collect()
}

/// this function may panic. In js that will abort execution
/// so we send it out of the wasm boundary
/// TODO try the wb_test func might be cheaper than closure
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
