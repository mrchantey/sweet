#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[sweet::test]
#[should_panic]
fn it_panics() { panic!("foo") }


