use crate::prelude::*;
use std::cell::RefCell;
use std::panic::PanicHookInfo;
use std::rc::Rc;


crate::scoped_thread_local! {
	static CURRENT_LISTENER: Rc<RefCell<Option<String>>>
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
		let payload = payload_to_string(info.payload());
		if !CURRENT_LISTENER.is_set() {
			// nobody is listening, must be a real one
			crate::log!("Uncaught Sweet Panic:\n{}", payload);
			return;
		} else {
			CURRENT_LISTENER.with(|last_panic| {
				*last_panic.borrow_mut() = Some(payload.to_string());
			});
		}
	}

	// pub fn get() -> String {
	// 	CURRENT_LISTENER.with(|last_panic| last_panic.borrow().clone())
	// }


	/// All sync and async wasm tests must be ran inside this scope.
	/// It will catch any panics and store them globally.
	/// Source of truth is the last panic that occured,
	/// # Returns
	/// an error if a panic occured
	pub fn with_scope<F>(func: F) -> TestOutput
	where
		F: FnOnce() -> Result<(), String>,
	{
		let output = Default::default();
		CURRENT_LISTENER.set(&output, || {
			let test_out = func();
			match (output.borrow_mut().take(), test_out) {
				(Some(err), _) => TestOutput::Panic(err),
				(None, Ok(_)) => TestOutput::Ok,
				(None, Err(err)) => TestOutput::Error(err),
			}
		})
	}

	// pub fn save_current_result(id: LibtestHash, failed: bool) {
	// 	let result = if failed { Err(Self::get()) } else { Ok(()) };
	// 	AsyncTestResults::set(id, result);
	// }
}
