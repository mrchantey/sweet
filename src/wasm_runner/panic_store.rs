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
		let payload = info.payload_as_str().unwrap_or("no panic message");
		if !CURRENT_LISTENER.is_set() {
			// nobody is listening, must be a real one
			crate::log!("Panic: {}", payload);
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


	pub fn set<F, R>(val: &Rc<RefCell<Option<String>>>, f: F) -> R
	where
		F: FnOnce() -> R,
	{
		CURRENT_LISTENER.set(val, f)
	}

	// pub fn save_current_result(id: LibtestHash, failed: bool) {
	// 	let result = if failed { Err(Self::get()) } else { Ok(()) };
	// 	AsyncTestResults::set(id, result);
	// }
}
