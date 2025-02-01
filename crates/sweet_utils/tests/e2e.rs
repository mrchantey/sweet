#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
#[cfg(target_arch = "wasm32")]
use sweet_test as sweet;
use sweet_utils::prelude::*;


/// `sweet_test` depends on `sweet_utils`
/// so we cant do sweet tests inside there
#[sweet_test::test]
async fn sleep_works() { sleep_millis(1).await; }
