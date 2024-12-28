use super::GlobalStore;
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
}


impl GlobalStore for PanicStore {
	fn var_name() -> &'static str { "__sweet_panic_output" }
}
