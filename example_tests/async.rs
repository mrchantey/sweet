#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
// use sweet::prelude::*;
#[test]
fn async_test() {
	sweet::prelude::AsyncTestPromises::store(23, my_test());
	sweet::prelude::panic_with_id(23);
}

async fn my_test() {
	wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(
		&mut |resolve, _| {
			web_sys::window()
				.unwrap()
				.set_timeout_with_callback_and_timeout_and_arguments_0(
					&resolve, 1000,
				)
				.unwrap();
		},
	))
	.await
	.unwrap();
}
