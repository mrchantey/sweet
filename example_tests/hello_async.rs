//! example usage of async tests
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use anyhow::Result;
use std::time::Duration;

#[sweet::test]
// #[should_panic]
async fn returns_err() -> Result<(), String> { Err("foo".to_string()) }

#[sweet::test]
#[should_panic]
#[ignore = "its just a dummy"]
async fn dummy1() {
	tokio::time::sleep(Duration::from_secs(1)).await;

	panic!("foo")
}

// #[cfg(target_arch = "wasm32")]
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
// #[should_panic]
async fn dummy2() {
	tokio::time::sleep(Duration::from_secs(1)).await;
	panic!("waddup")
}
#[sweet::test]
// #[should_panic]
async fn dummy3() { tokio::time::sleep(Duration::from_secs(1)).await; }
#[sweet::test]
// #[should_panic]
async fn dummy4() { tokio::time::sleep(Duration::from_secs(1)).await; }
#[sweet::test]
async fn dummy5() { tokio::time::sleep(Duration::from_secs(1)).await; }
// #[should_panic]

// #[sweet::test]
// #[should_panic]
// async fn it_panics() {
// 	if true {
// 		panic!("foo")
// 	}
// }


// /// tokio tests are just `#[test]` wrapped in a tokio runtime,
// /// of course they only run for native targets.
// ///
// /// Use `#[tokio::test]` when you need an isolated async runtime,
// /// 99% of the time you don't need it.
// #[cfg(not(target_arch = "wasm32"))]
// #[tokio::test]
// async fn its_tokio() {}
