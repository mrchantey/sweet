#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]



mod reqwest;


pub mod prelude {
	pub use crate::reqwest::*;
}


pub mod exports {
	pub use http;
	pub use http_body_util;
	pub use reqwest;
}
