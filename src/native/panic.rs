use crate::native::*;
// use anyhow::Result;
use futures::FutureExt;
use std::any::Any;
use std::panic::UnwindSafe;
use tokio::task::JoinError;


type PanicResult = Result<anyhow::Result<()>, Box<dyn Any + Send>>;

pub type ResultResult = anyhow::Result<anyhow::Result<()>>;

pub fn anyhow_panic(result: PanicResult) -> ResultResult {
	match result {
		Ok(result) => Ok(result),
		Err(e) => Err(anyhow::anyhow!(panic_info(e))),
	}
}
pub fn anyhow_tokio_join(
	result: Result<anyhow::Result<()>, JoinError>,
) -> ResultResult {
	match result {
		Ok(result) => Ok(result),
		Err(e) => Err(anyhow::anyhow!(panic_info(Box::new(e)))),
	}
}

pub fn unwrap_panic<F: FnOnce() -> anyhow::Result<R> + UnwindSafe, R>(
	func: F,
) -> Result<R, String> {
	flatten_panic(
		std::panic::catch_unwind(func).map(|r| r.map_err(|e| e.to_string())),
	)
}
pub fn unwrap_panic_str<F: FnOnce() -> Result<R, String> + UnwindSafe, R>(
	func: F,
) -> Result<R, String> {
	flatten_panic(std::panic::catch_unwind(func))
}
pub fn unwrap_panic_blocking(func: &fn() -> BoxedFuture) -> Result<(), String> {
	flatten_panic(std::panic::catch_unwind(move || {
		futures::executor::block_on(async {
			func().await.map_err(|e| e.to_string())
		})
	}))
}

pub async fn unwrap_panic_async(
	fut: BoxedFutureUnwindSafe,
) -> Result<(), String> {
	let val = fut.catch_unwind().await;
	let val = val.map(|r| r.map_err(|e| e.to_string()));
	flatten_panic(val)
}

pub fn flatten_panic<R>(
	result: std::thread::Result<Result<R, String>>,
) -> Result<R, String> {
	match result {
		Ok(result) => result,
		Err(e) => Err(panic_info(e)),
	}
}

fn panic_info(e: Box<dyn Any + Send>) -> String {
	match e.downcast::<String>() {
		Ok(v) => *v,
		Err(e) => match e.downcast::<&str>() {
			Ok(v) => v.to_string(),
			_ => "Unknown Source of Error".to_owned(),
		},
	}
}
