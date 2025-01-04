//! example usage of async tests
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use anyhow::Result;
use std::time::Duration;

#[sweet::test]
#[ignore = "it returns error"]
async fn returns_err() -> Result<(), String> { Err("foo".to_string()) }

#[sweet::test]
#[should_panic]
#[ignore = "its just a dummy"]
async fn dummy1() {
	// tokio::time::sleep(Duration::from_secs(1)).await;

	panic!("foo")
}

// #[cfg(target_arch = "wasm32")]
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
#[should_panic]
async fn dummy2() {
	tokio::time::sleep(Duration::from_secs(1)).await;
	panic!("waddup")
}
#[sweet::test]
// #[should_panic]
async fn dummy3() { sleep(Duration::from_secs(1)).await; }
#[sweet::test]
// #[should_panic]
async fn dummy4() { sleep(Duration::from_secs(1)).await; }
#[sweet::test]
#[should_panic]
async fn dummy5() {
	sleep(Duration::from_secs(1)).await;
	panic!("whaya");
}

// #[sweet::test]
// #[should_panic]
// async fn it_panics() {
// 	if true {
// 		panic!("foo")
// 	}
// }

#[cfg(target_arch = "wasm32")]
async fn sleep(duration: Duration) {
	use wasm_bindgen_futures::JsFuture;
	use web_sys::window;
	let window = window().unwrap();
	let promise = js_sys::Promise::new(&mut |resolve, _| {
		window
			.set_timeout_with_callback_and_timeout_and_arguments_0(
				&resolve,
				duration.as_millis() as i32,
			)
			.expect("should register `setTimeout` OK");
	});

	JsFuture::from(promise)
		.await
		.expect("should await `setTimeout` OK");
}

#[cfg(not(target_arch = "wasm32"))]
async fn sleep(duration: Duration) { tokio::time::sleep(duration).await; }
