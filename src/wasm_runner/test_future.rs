//! donation from the good folks at wasm-bindgen
//! https://github.com/rustwasm/wasm-bindgen/blob/24f20ae9bc480c5ad0778fdb1481eb23461f0d82/crates/test/src/rt/mod.rs#L764
use super::PanicStore;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use std::task::{
	self,
};
use wasm_bindgen::prelude::*;

pub struct TestFuture<F> {
	test: F,
}

impl<F> TestFuture<F> {
	pub fn new(test: F) -> Self { Self { test } }
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(catch)]
	fn __wbg_test_invoke(f: &mut dyn FnMut()) -> Result<(), JsValue>;
}

impl<F: Future<Output = Result<JsValue, JsValue>>> Future for TestFuture<F> {
	type Output = Result<(), String>;

	fn poll(
		self: Pin<&mut Self>,
		cx: &mut task::Context,
	) -> Poll<Self::Output> {
		// let output = self.output.clone();
		// Use `new_unchecked` here to project our own pin, and we never
		// move `test` so this should be safe
		let test = unsafe { Pin::map_unchecked_mut(self, |me| &mut me.test) };
		let mut future_poll_output = None;

		// did it panic during this poll
		let panic_result = PanicStore::with_scope(|| {
			let mut test = Some(test);
			__wbg_test_invoke(&mut || {
				let test = test.take().unwrap_throw();
				future_poll_output = Some(test.poll(cx))
			})
		});

		let next_state = match (panic_result, future_poll_output) {
			// case 1: its still going
			(_, Some(Poll::Pending)) => Poll::Pending,
			// case 2: it panicked
			(Err(e), _) => Poll::Ready(Err(e)),
			// case 3: it returned ok
			(_, Some(Poll::Ready(Ok(_)))) => Poll::Ready(Ok(())),
			// case 4: it returned an error but didnt panic
			(_, Some(Poll::Ready(Err(err)))) => {
				panic!("future returned an error but no panic registered, submit a bug report!\n{:?}", err)
			}
			// case 5: wtf
			(Ok(_), None) => wasm_bindgen::throw_str("invalid poll state"),
		};

		next_state
	}
}
