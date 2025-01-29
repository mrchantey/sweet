use crate::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Event;

pub struct EventRegistry;

thread_local! {
	static REGISTERED_EVENTS: RefCell<HashMap<(ElementIndex,String),Box<dyn Fn(JsValue)>>> = Default::default();
}

impl EventRegistry {
	fn trigger(key: &str, el_id: ElementIndex, value: JsValue) {
		REGISTERED_EVENTS.with(|current| {
			if let Some(func) = current.borrow().get(&(el_id, key.to_string()))
			{
				func(value);
			}
		});
	}

	fn register<T: 'static + JsCast>(
		key: &str,
		cx: &RsxContext,
		func: impl 'static + Fn(T),
	) {
		REGISTERED_EVENTS.with(|current| {
			current.borrow_mut().insert(
				(cx.html_element_index(), key.to_string()),
				Box::new(move |e: JsValue| {
					func(e.unchecked_into());
				}),
			);
		});
	}
	pub fn register_onclick(
		key: &str,
		cx: &RsxContext,
		value: impl 'static + Fn(Event),
	) {
		Self::register(key, cx, value);
	}
}

pub fn playback_prehydrate_events() -> ParseResult<()> {
	sweet_loader_extern::GLOBAL.with(|global| {
		let constants = CurrentHydrator::with(|h| h.html_constants().clone());
		// let event_handler =
		// 	js_sys::Reflect::get(&global, &constants.event_handler.into())
		// 		.map_err(|_| {
		// 			ParseError::Hydration("could not find event handler".into())
		// 		})?;
		let prehydrate_events =
			js_sys::Reflect::get(&global, &constants.prehydrate_events.into())
				.map_err(|_| {
					ParseError::Hydration("could not find event handler".into())
				})?;
		let prehydrate_events = js_sys::Array::from(&prehydrate_events);
		for item in prehydrate_events.iter() {
			let event_arr = js_sys::Array::from(&item);
			if event_arr.length() == 2 {
				let id =
					event_arr.get(0).as_f64().expect("bad event id") as usize;
				let event: Event = event_arr.get(1).unchecked_into();
				let event_type = format!("on{}", event.type_());
				EventRegistry::trigger(&event_type, id, event.unchecked_into());
			}
		}
		// 	} else {
		// 		return Err(ParseError::Hydration("bad event".into()));
		// 	}
		// }
		Ok(())
	})
}

// let event = event_arr.get(1);
// Self::trigger
// 	js_sys::Reflect::delete_property(
// 		&sweet.unchecked_ref(),
// 		&"uncanny".into(),
// 	)
// 	.unwrap_or_default();
// }
// let closure =
// 	Closure::wrap(Box::new(move |id: usize, evt: web_sys::Event| {
// 		func(id, evt);
// 	}) as Box<dyn FnMut(usize, web_sys::Event)>);

// js_sys::Reflect::set(&sweet, &"event".into(), &closure.as_ref())
// 	.unwrap_or_default();

// closure.forget();

pub mod sweet_loader_extern {
	use wasm_bindgen::prelude::*;
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen]
		#[wasm_bindgen(thread_local_v2,js_name = globalThis)]
		pub static GLOBAL: JsValue;
	}
}
