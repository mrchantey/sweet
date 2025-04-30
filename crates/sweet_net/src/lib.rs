#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
mod cross_fetch;
mod reqwest;
mod types;

pub mod prelude {
	pub use crate::cross_fetch::Request;
	pub use crate::cross_fetch::Response;
	pub use crate::reqwest::*;
	pub use crate::types::*;
}
pub mod exports {
	pub use http;
	pub use http_body_util;
	pub use reqwest;
}
