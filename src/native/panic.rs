// use anyhow::Result;
use std::any::Any;

pub fn panic_err_to_string(e: Box<dyn Any + Send>) -> String {
	match e.downcast::<String>() {
		Ok(v) => *v,
		Err(e) => match e.downcast::<&str>() {
			Ok(v) => v.to_string(),
			_ => "Failed to convert panic to string".to_owned(),
		},
	}
}
