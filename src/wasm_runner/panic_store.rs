use super::AsyncTestPanics;
use super::GlobalStore;
use anyhow::Result;
use std::panic::PanicHookInfo;


/// when a test panics, store it globally
/// and retrieve immediately after
pub struct PanicStore;

impl PanicStore {
	// it seems in wasm we can only set_hook once, otherwise
	// the setting of a hook itsself will panic
	pub fn panic_hook(info: &PanicHookInfo) {
		let payload = info.payload_as_str().unwrap_or("no panic message");
		// crate::log!("it panicked: {}", payload);
		Self::set_field("payload", payload);
	}

	pub fn get() -> String {
		Self::get_field("payload")
			.map(|val| val.as_string().unwrap())
			.unwrap_or_default()
	}

	pub fn save_current_as_test_failure(id: usize) -> Result<()> {
		let message = Self::get();
		AsyncTestPanics::set(id, message)
	}
}


impl GlobalStore for PanicStore {
	fn var_name() -> &'static str { "__sweet_panic_output" }
}
