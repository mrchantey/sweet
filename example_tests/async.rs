#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use sweet::prelude::*;
#[test]
fn fails() {
	
	// log_web("running");
	// blocking_wrapper("foobar".to_string());
	// log_web("done");

	// async fn its_async() {
	// 	use wasm_bindgen_futures::JsFuture;

	// 	log_web("Hello from async");
	// 	let promise = js_sys::Promise::new(&mut |resolve, _| {
	// 		web_sys::window()
	// 			.unwrap()
	// 			.set_timeout_with_callback_and_timeout_and_arguments_0(
	// 				&resolve, 10,
	// 			)
	// 			.unwrap();
	// 		log_web("running");
	// 	});
	// 	JsFuture::from(promise).await.unwrap();
	// 	log_web("done");
	// }

	// futures::executor::block_on(its_async());

	// sweet::prelude::expect(true).to_be_false().unwrap();
}


use std::sync::mpsc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub fn blocking_wrapper(input: String) -> String {
	let (tx, rx) = mpsc::channel();

	spawn_local(async move {
		let result = async_operation(input).await;
		tx.send(result).unwrap();
	});

	// Block until we get the result
	rx.recv().unwrap()
}

async fn async_operation(input: String) -> String {
	// Simulate some async work
	wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(
		&mut |resolve, _| {
			web_sys::window()
				.unwrap()
				.set_timeout_with_callback_and_timeout_and_arguments_0(
					&resolve, 1000, // 1 second delay
				)
				.unwrap();
		},
	))
	.await
	.unwrap();

	format!("Processed: {}", input)
}
