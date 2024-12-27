use crate::wasm::runner::log_web;
use wasm_bindgen::prelude::*;

pub fn wasm_panic() {
	fn panic_handling(mut message: String) {
		let should_panic = false;
		// let should_panic = CURRENT_OUTPUT.with(|output| {
		// 	let mut output = output.borrow_mut();
		// 	output.panic.push_str(&message);
		// 	output.should_panic
		// });

		// See https://github.com/rustwasm/console_error_panic_hook/blob/4dc30a5448ed3ffcfb961b1ad54d000cca881b84/src/lib.rs#L83-L123.
		if !should_panic {
			#[wasm_bindgen]
			extern "C" {
				type Error;

				#[wasm_bindgen(constructor)]
				fn new() -> Error;

				#[wasm_bindgen(method, getter)]
				fn stack(error: &Error) -> String;
			}

			message.push_str("\n\nStack:\n\n");
			let e = Error::new();
			let stack = e.stack();
			message.push_str(&stack);

			message.push_str("\n\n");

			log_web(&message);
		}
	}
	#[cfg(feature = "std")]
	static SET_HOOK: std::sync::Once = std::sync::Once::new();
	#[cfg(feature = "std")]
	SET_HOOK.call_once(|| {
		std::panic::set_hook(Box::new(|panic_info| {
			panic_handling(panic_info.to_string());
		}));
	});
	#[cfg(all(
		not(feature = "std"),
		target_arch = "wasm32",
		any(target_os = "unknown", target_os = "none")
	))]
	#[panic_handler]
	fn panic_handler(panic_info: &core::panic::PanicInfo<'_>) -> ! {
		panic_handling(panic_info.to_string());
		core::arch::wasm32::unreachable();
	}
}
