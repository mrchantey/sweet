//! used for cargo expand
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

#[sweet_test::test]
#[should_panic]
async fn it_panics() { panic!("foo") }
