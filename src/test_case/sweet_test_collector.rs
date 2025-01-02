use crate::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;

pub trait SweetestFuture:
	'static + Future<Output = Result<(), String>>
{
}
impl<T> SweetestFuture for T where
	T: 'static + Future<Output = Result<(), String>>
{
}


pub type SweetFutFunc =
	Box<dyn Send + Sync + Fn() -> Pin<Box<dyn SweetestFuture>>>;

type FutCell = Arc<Mutex<Option<SweetFutFunc>>>;

thread_local! {
	static FUTURE: FutCell = Arc::new(Mutex::new(None));
}

pub struct SweetTestCollector;

impl SweetTestCollector {
	/// # Panics
	/// If called outside of [`Self::set`]
	pub fn register<F: SweetestFuture>(fut: fn() -> F) {
		FUTURE.with(|current_fut| {
			*current_fut.lock().unwrap() =
				Some(Box::new(move || Box::pin(fut())));
		});
	}

	/// This function uses the Error type to represent
	/// that a future has been registered
	pub fn with_scope<F, R>(func: F) -> Result<R, SweetFutFunc>
	where
		F: FnOnce() -> R,
	{
		// let val = Arc::new(Mutex::new(None));
		FUTURE.with(|val| {
			let out = func();
			if let Some(fut) = val.lock().unwrap().take() {
				Err(fut)
			} else {
				Ok(out)
			}
		})
	}
}
