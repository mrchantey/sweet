use super::Hydrated;



pub struct SweetLoader;

impl SweetLoader {
	pub fn load(self, hydrated: Hydrated) {
		



	}
}




pub mod sweet_loader_extern {
	use wasm_bindgen::prelude::*;
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen]
		#[wasm_bindgen(js_name = _sweet_event)]
		pub static SWEET_EVENT_HANDLER: JsValue;
		#[wasm_bindgen]
		#[wasm_bindgen(js_name = globalThis)]
		pub static GLOBAL: JsValue;
	}
}


// #[wasm_bindgen(main)]
// pub fn main() {}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {}
}
