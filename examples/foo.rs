use backtrace::Backtrace;
use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;


// #[wasm_bindgen]
pub fn main() {
	console_error_panic_hook::set_once();
	// let a = read_file("examples/foods.rs");
	// let a = sweet::prelude::wasm_fs::read_file("examples/foo.rs");
	// .as_string()
	// .unwrap();
	bar();
}

fn bar() {
	let backtrace = Backtrace::new();
	let backtrace = backtrace.frames().len();
	sweet::log!("backtrace: {:?}", backtrace);
}
