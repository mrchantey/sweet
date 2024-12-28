use super::SerdeTestDesc;
use js_sys::Object;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestDescriptions(HashMap<usize, SerdeTestDesc>);


impl GlobalStore for TestDescriptions {
	fn var_name() -> &'static str { "__sweet_pending_test_descriptions" }
}


pub trait GlobalStore: Sized {
	fn var_name() -> &'static str;

	fn get() -> Object {
		let var_name = Self::var_name();
		let window = web_sys::window().expect("no global window exists");
		let obj = js_sys::Reflect::get(&window, &var_name.into()).unwrap();
		if obj.is_undefined() {
			let obj = Object::new();
			js_sys::Reflect::set(&window, &var_name.into(), &obj).unwrap();
			obj
		} else {
			obj.unchecked_into()
		}
	}


	fn set_field(key: impl Into<JsValue>, val: impl Into<JsValue>) {
		let obj = Self::get();
		set_field(&obj, key, val);
	}
	fn get_field(key: impl Into<JsValue>) -> Result<JsValue, JsValue> {
		let obj = Self::get();
		js_sys::Reflect::get(&obj, &key.into())
	}

	fn print() {
		let obj = Self::get();
		crate::log!("{}: {:?}", Self::var_name(), obj);
	}
}


fn set_field(
	obj: &js_sys::Object,
	key: impl Into<JsValue>,
	val: impl Into<JsValue>,
) {
	js_sys::Reflect::set(obj, &key.into(), &val.into()).unwrap();
}
