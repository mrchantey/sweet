#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
// use sweet::prelude::*;
#[test]
fn async_test() {
	let id = 0;
	sweet::prelude::AsyncTestPromises::store(id, my_test(id));
	sweet::prelude::panic_with_id(id);
}

#[test]
fn async_test2() {
	let id = 1;
	sweet::prelude::AsyncTestPromises::store(id, my_test(id));
	sweet::prelude::panic_with_id(id);
}

async fn my_test(id:usize) {
	wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(
		&mut |resolve, _| {
			web_sys::window()
				.unwrap()
				.set_timeout_with_callback_and_timeout_and_arguments_0(
					&resolve, 100,
				)
				.unwrap();
		},
	))
	.await
	.unwrap();
	panic!("pizza football {}",id);
}
