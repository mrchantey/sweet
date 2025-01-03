//! donation from the good folks at wasm-bindgen 🙏
//! https://github.com/rustwasm/wasm-bindgen/blob/24f20ae9bc480c5ad0778fdb1481eb23461f0d82/crates/test/src/rt/mod.rs#L764
use crate::prelude::*;
use flume::Sender;
use std::future::Future;
use std::pin::Pin;
use std::task;
use std::task::Poll;
use test::TestDesc;
use wasm_bindgen::prelude::*;

pub struct TestFuture<F> {
	desc: TestDesc,
	result_tx: Sender<TestDescAndResult>,
	test: F,
}

impl<F> TestFuture<F> {
	pub fn new(
		desc: TestDesc,
		result_tx: Sender<TestDescAndResult>,
		test: F,
	) -> Self {
		Self {
			desc,
			result_tx,
			test,
		}
	}
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(catch)]
	fn __wbg_test_invoke(f: &mut dyn FnMut()) -> Result<(), JsValue>;
}

impl<F: Future<Output = Result<JsValue, JsValue>>> Future for TestFuture<F> {
	type Output = ();

	fn poll(
		self: Pin<&mut Self>,
		cx: &mut task::Context,
	) -> Poll<Self::Output> {
		// let output = self.output.clone();
		// Use `new_unchecked` here to project our own pin, and we never
		// move `test` so this should be safe

		let self_mut = unsafe { self.get_unchecked_mut() };
		let test = unsafe { Pin::new_unchecked(&mut self_mut.test) };
		let desc = &self_mut.desc;
		let result_tx = &self_mut.result_tx;
		let mut future_poll_output = None;

		// did it panic during this poll
		let panic_output = PanicStore::with_scope(desc, || {
			let mut test = Some(test);
			__wbg_test_invoke(&mut || {
				let test = test.take().unwrap_throw();
				future_poll_output = Some(test.poll(cx))
			})
		});

		match panic_output {
			PanicStoreOut::Panicked(test_desc_and_result) => {
				result_tx
					.send(test_desc_and_result)
					.expect("channel was dropped");
				return Poll::Ready(());
			}
			PanicStoreOut::Ok(_) => {
				// could be pending
			}
		}


		match future_poll_output {
			Some(Poll::Pending) => Poll::Pending,
			_ => {
				let test_result = TestResult::from_test_result(Ok(()), &desc);
				result_tx
					.send(TestDescAndResult::new(desc.clone(), test_result))
					.expect("channel was dropped");

				Poll::Ready(())
			}
		}
	}
}