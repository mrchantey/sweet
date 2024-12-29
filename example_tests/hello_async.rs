#![allow(unused)]
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use std::cell::RefCell;
use std::rc::Rc;
use sweet::prelude::*;
// use sweet::prelude::*;
#[test]
// #[should_panic]
fn async_test() {
	SweetTestCollector::register(Box::pin(async {
		// this is better than how #[test] does it anyway
		// #[test] discards the error and returns a useless one
		if let Err(err) = returns_err().await {
			panic!("{:?}", err);
		}
	}));
	// SweetTestCollector::register(Box::pin(my_test()));
	// sweet::prelude::panic_with_id(id);
}

async fn panics() -> Result<(), String> { panic!("foo") }
async fn returns_ok() -> Result<(), String> { Ok(()) }
async fn returns_err() -> Result<(), String> { Err("foo".to_string()) }
