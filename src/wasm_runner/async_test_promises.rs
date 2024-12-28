use super::GlobalStore;
use crate::libtest::libtest_result_to_panic;
use std::future::Future;
use wasm_bindgen::JsValue;



#[derive(Debug, Default, Clone)]
pub struct AsyncTestPromises;


impl AsyncTestPromises {
	pub fn store<O: UnitOrResult, T: Future<Output = O> + 'static>(
		id: usize,
		fut: T,
	) {
		let prom = wasm_bindgen_futures::future_to_promise({
			async move {
				fut.await;
				Ok(JsValue::NULL)
			}
		});
		Self::set_field(id, prom);
	}
}


impl GlobalStore for AsyncTestPromises {
	fn var_name() -> &'static str { "__sweet_pending_test_promises" }
}


pub trait UnitOrResult {
	fn unwrap(self) -> ();
}
impl UnitOrResult for () {
	fn unwrap(self) -> () { self }
}
impl<T> UnitOrResult for Result<(), T> {
	fn unwrap(self) -> () { libtest_result_to_panic(self); }
}
