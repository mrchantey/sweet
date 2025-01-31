#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
use sweet_utils::prelude::*;

#[sweet_test::test]
async fn sleep_works() { sleep_millis(1).await; }
