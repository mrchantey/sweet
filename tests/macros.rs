//! example usage of async tests
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use anyhow::Result;
use std::time::Duration;

#[test]
#[ignore]
fn its_ignored_sync() { panic!("foo") }
#[test]
#[should_panic = "cos its fun"]
fn it_panics_sync() { panic!("foo") }

#[sweet::test]
async fn it_passes() {}
#[sweet::test]
async fn it_returns_ok() -> Result<(), String> { Ok(()) }
#[sweet::test]
#[ignore = "it returns error"]
async fn it_returns_err() -> Result<(), String> { Err("foo".to_string()) }

#[sweet::test]
#[should_panic]
async fn it_panics() { panic!("foo") }

// #[cfg(target_arch = "wasm32")]
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
#[should_panic]
async fn it_tokio_waits_then_panics() {
	tokio::time::sleep(Duration::from_secs(1)).await;
	panic!("waddup")
}
#[sweet::test]
// #[should_panic]
async fn it_sleeps() { sleep(Duration::from_secs(1)).await; }

#[sweet::test]
#[should_panic]
async fn it_sleeps_then_panics() {
	sleep(Duration::from_secs(1)).await;
	panic!("waddup")
}

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
