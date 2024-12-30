#![cfg_attr(rustfmt, rustfmt_skip)]
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[test]
// #[should_panic]
fn succeeds() { 
	sweet::prelude::expect(true).to_be_true().unwrap();
}
// #[test]
// // #[should_panic]
// fn errors()->anyhow::Result<()>{ 
// 	sweet::prelude::expect(true).to_be_false()
// }

#[test]
// #[should_panic]
fn fails2() { 
	sweet::prelude::expect(true).to_be_false().unwrap();
}
