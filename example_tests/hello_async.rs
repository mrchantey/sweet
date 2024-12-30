//! example usage of async tests
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use sweet::prelude::*;

#[sweet::test]
#[should_panic]
async fn returns_err() -> Result<(), String> { Err("foo".to_string()) }

#[sweet::test]
#[should_panic]
async fn panics() { panic!("foo") }


/// tokio tests are just `#[test]` wrapped in a tokio runtime,
/// of course they only run for native targets.
///
/// Use `#[tokio::test]` when you need an isolated async runtime,
/// 99% of the time you don't need it.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn its_tokio() {}
