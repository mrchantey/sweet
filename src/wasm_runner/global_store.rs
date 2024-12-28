use super::SerdeTestDesc;
use anyhow::Result;
use js_sys::Object;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestDescriptions(HashMap<usize, SerdeTestDesc>);


impl GlobalStore for TestDescriptions {
	fn var_name() -> &'static str { "__sweet_pending_test_descriptions" }
}


pub trait GlobalStore: Sized {
	// type Value:

	fn var_name() -> &'static str;

	fn get_store_object() -> Object {
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

	fn exists() -> bool {
		let var_name = Self::var_name();
		let window = web_sys::window().expect("no global window exists");
		let obj = js_sys::Reflect::get(&window, &var_name.into()).unwrap();
		!obj.is_undefined()
	}


	fn set_field(key: impl Into<JsValue>, val: impl Into<JsValue>) {
		let obj = Self::get_store_object();
		set_field(&obj, key, val);
	}
	fn get_field(key: impl Into<JsValue>) -> Result<JsValue, JsValue> {
		let obj = Self::get_store_object();
		js_sys::Reflect::get(&obj, &key.into())
	}

	fn set_serde(
		key: impl Into<JsValue>,
		value: &impl Serialize,
	) -> Result<()> {
		let serde = serde_json::to_string(&value)?;
		Self::set_field(key, serde);
		Ok(())
	}
	fn get_serde<T: DeserializeOwned>(
		key: impl std::fmt::Debug + Clone + Into<JsValue>,
	) -> Result<T> {
		let serde = Self::get_field(key.clone()).map_err(|_| {
			anyhow::anyhow!(
				"global store value not found {}: {:?}",
				Self::var_name(),
				key
			)
		})?;
		let serde: String = serde.as_string().ok_or_else(|| {
			anyhow::anyhow!(
				"global store value is not a serde string: {:?}",
				key
			)
		})?;
		let value = serde_json::from_str(&serde)?;
		Ok(value)
	}


	fn collect() -> Result<HashMap<String, JsValue>> {
		let obj = Self::get_store_object();
		let keys = js_sys::Reflect::own_keys(&obj).unwrap();
		keys.iter()
			.map(|key| {
				let value = Self::get_field(&key).unwrap();
				let key = key.as_string().unwrap();
				Ok((key, value))
			})
			.collect()
	}
	fn collect_serde<T: DeserializeOwned>() -> Result<HashMap<String, T>> {
		let obj = Self::get_store_object();
		let keys = js_sys::Reflect::own_keys(&obj).unwrap();
		keys.iter()
			.map(|key| {
				let value = Self::get_serde(&key)?;
				let key = key.as_string().unwrap();
				Ok((key, value))
			})
			.collect()
	}

	fn print() {
		let obj = Self::get_store_object();
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
