use crate::prelude::*;
use std::cell::RefCell;
use std::panic::PanicHookInfo;
use std::rc::Rc;
use test::TestDesc;

pub enum PanicStoreOut<T> {
	Panicked(TestDescAndResult),
	Ok(T),
}


crate::scoped_thread_local! {
	static CURRENT_LISTENER: Rc<RefCell<(TestDesc,Option<TestResult>)>>
}

/// when a test panics, store it globally
/// and retrieve immediately after
pub struct PanicStore;

impl PanicStore {
	// it seems in wasm we can only set_hook once, otherwise
	// the setting of a hook itsself will panic
	/// This will be called from inside thie function
	/// at some point duing a Scoped Set
	pub fn panic_hook(info: &PanicHookInfo) {
		if !CURRENT_LISTENER.is_set() {
			// nobody is listening, must be a real one
			let payload = payload_to_string(info.payload());
			crate::log!("Uncaught Sweet Panic:\n{}", payload);
			return;
		} else {
			CURRENT_LISTENER.with(|last_panic| {
				let result =
					TestResult::from_panic(info, &last_panic.borrow().0);
				last_panic.borrow_mut().1 = Some(result);
			});
		}
	}

	// pub fn get() -> String {
	// 	CURRENT_LISTENER.with(|last_panic| last_panic.borrow().clone())
	// }



	/// if the function panics, and it should not have
	/// this will return None and emit a result.
	/// Otherwise deal with the function
	pub fn with_scope<F, R>(desc: &TestDesc, func: F) -> PanicStoreOut<R>
	where
		F: FnOnce() -> R,
	{
		let output = Rc::new(RefCell::new((desc.clone(), None)));
		CURRENT_LISTENER.set(&output, || {
			let test_out = func();
			match (output.borrow_mut().1.take(), test_out) {
				(Some(panic_result), _) => PanicStoreOut::Panicked(
					TestDescAndResult::new(desc.clone(), panic_result),
				),
				(None, result) => PanicStoreOut::Ok(result),
			}
		})
	}

	// pub fn save_current_result(id: LibtestHash, failed: bool) {
	// 	let result = if failed { Err(Self::get()) } else { Ok(()) };
	// 	AsyncTestResults::set(id, result);
	// }
}