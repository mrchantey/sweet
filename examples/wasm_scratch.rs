#![feature(panic_payload_as_str)]
use forky::prelude::*;
use sweet::wasm::runner::log_web;
use wasm_bindgen::prelude::*;


fn main() {
	let testid = 69;
	std::panic::set_hook(Box::new(move |panic_info| {
		let payload = panic_info.payload_as_str().unwrap_or("no panic message");
		set_test_output(testid, payload);
	}));
	// 1. create a closure that will panic
	let closure = Closure::from_func_no_args(will_panic);
	let func: &js_sys::Function = closure.as_ref().unchecked_ref();

	let result = func.call0(&JsValue::NULL);
	match result {
		Ok(_) => log_web("Ok"),
		// the error returned from a panic is just an Unreachable with backtrace
		Err(_) => {
			let out = get_test_output(testid);
			log_web(&format!("Failed: {:?}", out));
		}
	}
}

fn will_panic() {
	log_web("im gonna panic");
	panic!("🚀🚀🚀");
}

fn set_test_output(id: usize, value: &str) {
	let window = web_sys::window().expect("no global window exists");
	js_sys::Reflect::set(
		&window,
		&JsValue::from_str(format!("test_output_{}", id).as_str()),
		&JsValue::from_str(value),
	)
	.unwrap();
}

fn get_test_output(id: usize) -> String {
	let window = web_sys::window().expect("no global window exists");
	let output = js_sys::Reflect::get(
		&window,
		&JsValue::from_str(format!("test_output_{}", id).as_str()),
	)
	.unwrap();
	output.as_string().unwrap()
}
