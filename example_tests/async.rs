#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use sweet::prelude::LibtestHash;
// use sweet::prelude::*;
#[test]
// #[should_panic]
fn async_test() {
	let id = LibtestHash::new(file!(), (line!() as usize) - 1);
	sweet::prelude::AsyncTestPromises::store(id, my_test(id));
	// sweet::prelude::panic_with_id(id);
	sweet::log!("async_test1 {}", id);
}

#[test]
// #[should_panic]
fn async_test2() {
	let id = LibtestHash::new(file!(), (line!() as usize) - 1);
	sweet::log!("async_test2 {}", id);
	sweet::prelude::AsyncTestPromises::store(id, my_test(id));
	// sweet::prelude::panic_with_id(id);
}

async fn my_test(id: LibtestHash) {
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
	panic!("pizza football {}", id);
}
