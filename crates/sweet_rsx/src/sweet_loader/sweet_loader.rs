use super::Hydrated;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Event;

#[derive(Default)]
pub struct SweetLoader;

impl SweetLoader {
	pub fn load(self, mut hydrated: Hydrated) {
		let mut func = move |id: usize, evt: Event| {
			let handler = hydrated.events.get_mut(id).expect(&format!(
				"failed to get hydrated event with id: {}",
				id
			));
			handler(evt);
		};


		sweet_loader_extern::GLOBAL.with(|global| {
			let sweet = match js_sys::Reflect::get(&global, &"_sweet".into()) {
				Ok(s) if !s.is_undefined() => s,
				_ => {
					let obj = js_sys::Object::new();

					js_sys::Reflect::set(&global, &"_sweet".into(), &obj)
						.unwrap_or_default();
					obj.into()
				}
			};
			if let Ok(uncanny) = js_sys::Reflect::get(&sweet, &"uncanny".into())
			{
				let uncanny_array = js_sys::Array::from(&uncanny);
				for item in uncanny_array.iter() {
					let event_arr = js_sys::Array::from(&item);
					if event_arr.length() == 2 {
						let id =
							event_arr.get(0).as_f64().unwrap_or(0.0) as usize;
						let event = web_sys::Event::from(event_arr.get(1));
						func(id, event);
					}
				}
				js_sys::Reflect::delete_property(
					&sweet.unchecked_ref(),
					&"uncanny".into(),
				)
				.unwrap_or_default();
			}
			let closure =
				Closure::wrap(Box::new(move |id: usize, evt: web_sys::Event| {
					func(id, evt);
				}) as Box<dyn FnMut(usize, web_sys::Event)>);

			js_sys::Reflect::set(&sweet, &"event".into(), &closure.as_ref())
				.unwrap_or_default();

			closure.forget();
		});
	}
}


pub mod sweet_loader_extern {
	use wasm_bindgen::prelude::*;
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen]
		#[wasm_bindgen(thread_local_v2,js_name = globalThis)]
		pub static GLOBAL: JsValue;
	}
}
