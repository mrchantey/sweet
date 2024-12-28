use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen_test::wasm_bindgen_test]
async fn my_async_test() {
	// Create a promise that is ready on the next tick of the micro task queue.
	let promise = js_sys::Promise::resolve(&JsValue::from(42));

	// Convert that promise into a future and make the test wait on it.
	let x = JsFuture::from(promise).await.unwrap();
	assert_eq!(x, 42);
}
