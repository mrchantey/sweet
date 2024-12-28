use super::GlobalStore;
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
		// async fn await_no_abort(fut: T) -> Result<(), JsValue> {
		// 	// Ok(())
		// 	// let (promise, closure) = {
		// 	// 	// let mut closure_holder = None;
		// 	// 	let promise = Promise::new(&mut move |resolve, _reject| {
		// 	// 		let prom = prom.clone();
		// 	// 		let closure = Closure::once(move || {
		// 	// 			// let prom = prom.clone();
		// 	// 			wasm_bindgen_futures::spawn_local(async move {
		// 	// 				let _ = JsFuture::from(prom).await;
		// 	// 				resolve.call0(&JsValue::NULL).unwrap();
		// 	// 			});
		// 	// 		});
		// 	// 		// closure_holder = Some(closure);
		// 	// 	});
		// 	// 	(promise, ())
		// 	// };

		// 	// // Keep closure alive until promise completes
		// 	// let result = JsFuture::from(promise).await;
		// 	// // drop(closure);
		// 	// result.map(|_| ())

		// 	let escaped_prom = wasm_bindgen_futures::future_to_promise({
		// 		async move {
		// 			libtest_result_to_panic(JsFuture::from(prom).await);
		// 			// libtest_result_to_panic(JsFuture::from(prom).await);
		// 			Ok(JsValue::NULL)
		// 		}
		// 	});

		// 	JsFuture::from(escaped_prom).await.map(|_| ())
		// }


		let prom = wasm_bindgen_futures::future_to_promise({
			async move {
				fut.await.unwrap();
				Ok(JsValue::NULL)
			}
		});

		// let _prom2 prom.catch(&Closure::once(|err| {
		// 	crate::log!("{:?}", err);
		// }));

		let fut = JsFuture::from(prom);

		let escaped_prom = wasm_bindgen_futures::future_to_promise(fut);

		Self::set_field(id, escaped_prom);
	}


	pub async fn collect() -> Result<(), JsValue> {
		let obj = Self::get_store_object();
		let keys = js_sys::Reflect::own_keys(&obj)?;
		let promises = keys
			.iter()
			.map(|key| {
				let prom = Self::get_field(key)?;
				let prom: Promise = prom.unchecked_into();
				// crate::log!("{:?}", prom);
				Ok(await_no_abort(prom))
			})
			.collect::<Result<Vec<_>, JsValue>>()?;

		crate::log!("hello");
		for prom in promises {
			let _ = prom.await.ok();
		}
		crate::log!("world🚀🚀🚀");

		// let results = try_join_all(promises).await.ok();

		// crate::log!("{:?}", results);

		Ok(())
	}
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
