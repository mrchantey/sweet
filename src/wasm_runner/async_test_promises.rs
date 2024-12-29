use super::anyhow_to_jsvalue;
use super::AsyncTestResults;
use crate::libtest::LibtestHash;
use crate::libtest::TestDescExt;
use futures::future::try_join_all;
use js_sys::Promise;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::rc::Rc;
use test::TestDesc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

thread_local! {
	static STORE: Rc<RefCell<HashMap<LibtestHash,Promise>>> = Default::default();
}


#[derive(Debug, Default, Clone)]
pub struct AsyncTestPromises;

impl AsyncTestPromises {
	pub fn has(id: LibtestHash) -> bool {
		STORE.with(|store| store.borrow().contains_key(&id))
	}


	pub fn store<O: UnitOrResult, T: Future<Output = O> + 'static>(
		id: LibtestHash,
		fut: T,
	) {
		// let prom = wasm_bindgen_futures::future_to_promise({
		// 	// async move {
		// 	TestFuture::new(id, async move {
		// 		fut.await.unwrap_libtest_err();
		// 		Ok(JsValue::NULL)
		// 	})
		// 	// fut.await.unwrap();
		// 	// }
		// });

		// STORE.with(|store| {
		// 	store.borrow_mut().insert(id, prom);
		// });
	}


	pub async fn await_and_collect(
	) -> Result<Vec<(TestDesc, Result<(), String>)>, JsValue> {
		let promises = STORE
			.with(|store| store.borrow_mut().drain().collect::<Vec<_>>())
			.into_iter()
			.map(|(_, prom)| {
				let prom: Promise = prom.unchecked_into();
				Ok(JsFuture::from(prom))
			})
			.collect::<Result<Vec<_>, JsValue>>()?;
		// 💀 poison, do not consume.
		// failed promises panicked so these results are invalid memory
		let _poison_results = try_join_all(promises).await;

		// Ok(Default::default())
		AsyncTestResults::collect().map_err(anyhow_to_jsvalue)
	}
}


pub trait UnitOrResult {
	fn unwrap_libtest_err(self) -> ();
}
impl UnitOrResult for () {
	fn unwrap_libtest_err(self) -> () { self }
}
impl<T> UnitOrResult for Result<(), T> {
	fn unwrap_libtest_err(self) -> () { TestDescExt::result_to_panic(self); }
}
