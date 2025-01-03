use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;


// #[wasm_bindgen]
pub fn main() {
	sweet::log!("waddup");

	let (send, recv) = flume::unbounded();

	send.send("pizza").unwrap();

	sweet::log!("chicken");
	while let Ok(msg) = recv.try_recv() {
		sweet::log!("msg: {}", msg);
	}

	let a = Arc::new(Mutex::new(0));
	sweet::log!("bazz: ");
	let foo = a.lock().unwrap();
	sweet::log!("foo: {}", foo);
}
