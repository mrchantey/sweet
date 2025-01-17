//! example usage of async tests
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[sweet::test]
#[ignore = "it returns error"]
async fn returns_err() -> Result<(), String> { Err("foo".to_string()) }

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
#[should_panic]
async fn dummy2() {
	sweet_core::sleep_secs(1).await;
	panic!("waddup")
}
#[sweet::test]
// #[should_panic]
async fn dummy3() { sweet_core::sleep_secs(1).await; }
#[sweet::test]
// #[should_panic]
async fn dummy4() { sweet_core::sleep_secs(1).await; }
#[sweet::test]
#[should_panic]
async fn dummy5() {
	sweet_core::sleep_secs(1).await;
	panic!("whaya");
}
