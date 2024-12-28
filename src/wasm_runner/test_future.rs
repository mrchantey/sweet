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
	// output: Rc<RefCell<String>>,
	id: usize,
	test: F,
}

impl<F> TestFuture<F> {
	pub fn new(id: usize, test: F) -> Self { Self { id, test } }
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
		// let result = CURRENT_OUTPUT.set(&output, || {
		let mut test = Some(test);
		let result = __wbg_test_invoke(&mut || {
			let test = test.take().unwrap_throw();
			future_output = Some(test.poll(cx))
		});

		// useless unreachable() error
		if let Err(_) = &result {
			PanicStore::save_current_as_test_failure(id).unwrap_or_else(|e| {
				wasm_bindgen::throw_str(&format!("{:?}", e))
			});
		}

		// crate::log!("hello {:?}", result);
		// });
		match (result, future_output) {
			(_, Some(Poll::Ready(result))) => Poll::Ready(result),
			(_, Some(Poll::Pending)) => Poll::Pending,
			(Err(e), _) => Poll::Ready(Err(e)),
			(Ok(_), None) => wasm_bindgen::throw_str("invalid poll state"),
		}
	}
}
