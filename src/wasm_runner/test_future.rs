//! donation from the good folks at wasm-bindgen
//! https://github.com/rustwasm/wasm-bindgen/blob/24f20ae9bc480c5ad0778fdb1481eb23461f0d82/crates/test/src/rt/mod.rs#L764
use super::PanicStore;
use crate::libtest::LibtestHash;
use crate::prelude::AsyncTestResults;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use std::task::{
	self,
};
use wasm_bindgen::prelude::*;

pub struct TestFuture<F> {
	// output: Rc<RefCell<String>>,
	id: LibtestHash,
	test: F,
}

impl<F> TestFuture<F> {
	pub fn new(id: LibtestHash, test: F) -> Self { Self { id, test } }
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(catch)]
	fn __wbg_test_invoke(f: &mut dyn FnMut()) -> Result<(), JsValue>;
}

impl<F: Future<Output = Result<JsValue, JsValue>>> Future for TestFuture<F> {
	type Output = F::Output;

	fn poll(
		self: Pin<&mut Self>,
		cx: &mut task::Context,
	) -> Poll<Self::Output> {
		// let output = self.output.clone();
		// Use `new_unchecked` here to project our own pin, and we never
		// move `test` so this should be safe
		let id = self.id;
		let test = unsafe { Pin::map_unchecked_mut(self, |me| &mut me.test) };
		let mut future_output = None;
		let output = Default::default();

		// did it panic during this poll
		let panic_result = PanicStore::set(&output, || {
			let mut test = Some(test);
			__wbg_test_invoke(&mut || {
				let test = test.take().unwrap_throw();
				future_output = Some(test.poll(cx))
			})
		});
		// this is wrong, result does not indicate we're done
		// PanicStore::save_current_result(id, result.is_err());


		let next_state = match (panic_result, future_output) {
			(_, Some(Poll::Ready(result))) => Poll::Ready(result),
			(_, Some(Poll::Pending)) => Poll::Pending,
			(Err(e), _) => Poll::Ready(Err(e)),
			(Ok(_), None) => wasm_bindgen::throw_str("invalid poll state"),
		};

		match (&next_state, output.borrow().as_ref()) {
			(Poll::Ready(_), Some(panic_err)) => {
				AsyncTestResults::set(id, Err(panic_err.clone()));
			}
			(Poll::Ready(_), None) => {
				AsyncTestResults::set(id, Ok(()));
			}
			_ => {
				// pending
			}
		}
		next_state
	}
}
