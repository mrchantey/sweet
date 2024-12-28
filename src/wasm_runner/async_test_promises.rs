use super::anyhow_to_jsvalue;
use super::AsyncTestPanics;
use super::GlobalStore;
use super::SerdeTestDesc;
use super::TestFuture;
use crate::libtest::libtest_result_to_panic;
use forky::web::ClosureFnMutT2Ext;
use futures::future::try_join_all;
use js_sys::Promise;
use std::future::Future;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;



#[derive(Debug, Default, Clone)]
pub struct AsyncTestPromises;

impl GlobalStore for AsyncTestPromises {
	fn var_name() -> &'static str { "__sweet_pending_test_promises" }
}

impl AsyncTestPromises {
	pub fn store<O: UnitOrResult, T: Future<Output = O> + 'static>(
		id: usize,
		fut: T,
	) {
		let prom = wasm_bindgen_futures::future_to_promise({
			// async move {
			TestFuture::new(id, async move {
				fut.await.unwrap_libtest_err();
				Ok(JsValue::NULL)
			})
			// fut.await.unwrap();
			// }
		});

		Self::set_field(id, prom);
	}


	pub async fn await_and_collect(
	) -> Result<Vec<(SerdeTestDesc, String)>, JsValue> {
		let promises = Self::collect()
			.unwrap()
			.into_iter()
			.map(|(_, prom)| {
				let prom: Promise = prom.unchecked_into();
				Ok(JsFuture::from(prom))
			})
			.collect::<Result<Vec<_>, JsValue>>()?;
		// 💀 poison, do not consume.
		// failed promises panicked so these results are invalid memory
		let _poison_results = try_join_all(promises).await;

		Ok(Default::default())
		// AsyncTestPanics::collect().map_err(anyhow_to_jsvalue)
	}
}


pub trait UnitOrResult {
	fn unwrap_libtest_err(self) -> ();
}
impl UnitOrResult for () {
	fn unwrap_libtest_err(self) -> () { self }
}
impl<T> UnitOrResult for Result<(), T> {
	fn unwrap_libtest_err(self) -> () { libtest_result_to_panic(self); }
}
