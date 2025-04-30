use super::*;
use crate::prelude::*;
use http::HeaderMap;
use serde::Serialize;
use std::borrow::Cow;
use std::str::FromStr;
use std::time::Duration;

/// A cross-platform fetch function that works on both native and wasm targets.
/// While `reqwest` does work on wasm it still is a heavy build, instead cross-fetch
/// just uses fetch directly.
///
/// ## Targets
///
/// - Native: `reqwest`
/// - Wasm: `fetch`
#[derive(Debug, Clone)]
pub struct Request<'a> {
	pub url: Cow<'a, str>,
	pub method: HttpMethod,
	pub headers: HeaderMap,
	pub timeout: Option<Duration>,
	pub body: Option<Vec<u8>>,
}

impl<'a> Request<'a> {
	pub fn new(url: impl Into<Cow<'a, str>>) -> Self {
		Self {
			url: url.into(),
			method: HttpMethod::Get,
			headers: HeaderMap::new(),
			timeout: None,
			body: None,
		}
	}
	pub fn method(mut self, method: HttpMethod) -> Self {
		self.method = method;
		self
	}
	pub fn header(mut self, key: &str, value: &str) -> Result<Self> {
		self.headers.insert(
			http::header::HeaderName::from_str(key).map_err(|_| {
				Error::Serialization(format!("Invalid header name: {}", key))
			})?,
			http::header::HeaderValue::from_str(value).map_err(|_| {
				Error::Serialization(format!("Invalid header value: {}", value))
			})?,
		);
		Ok(self)
	}
	/// Serailizes the body to JSON and sets the `Content-Type` header to `application/json`.
	pub fn body<T: Serialize>(mut self, body: T) -> Result<Self> {
		self.body = Some(serde_json::to_vec(&body).map_err(|e| {
			Error::Serialization(format!("Failed to serialize body: {}", e))
		})?);
		self.headers.insert(
			http::header::CONTENT_TYPE,
			http::header::HeaderValue::from_static("application/json"),
		);
		Ok(self)
	}
	/// Sets the body to a raw byte array and sets the `Content-Type` header to `application/octet-stream`.
	pub fn body_raw(mut self, body: Vec<u8>) -> Self {
		self.body = Some(body);
		self.headers.insert(
			http::header::CONTENT_TYPE,
			http::header::HeaderValue::from_static("application/octet-stream"),
		);
		self
	}
}

#[cfg(test)]
mod test {
	use crate::cross_fetch::ResponseInner;
	use crate::prelude::*;
	use sweet_test::prelude::*;
	use sweet_utils::prelude::*;

	const HTTPBIN: &str = "https://httpbin.org";

	#[sweet_test::test]
	async fn works() {
		Request::new("https://example.com")
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);
	}

	#[sweet_test::test]
	async fn get_works() {
		Request::new(format!("{}/get", HTTPBIN))
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);
	}

	#[sweet_test::test]
	async fn post_json_works() {
		Request::new(format!("{}/post", HTTPBIN))
			.method(HttpMethod::Post)
			.body(&serde_json::json!({"foo": "bar"}))
			.unwrap()
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);
	}

	#[sweet_test::test]
	async fn custom_header_works() {
		Request::new(format!("{}/headers", HTTPBIN))
			.header("X-Foo", "Bar")
			.unwrap()
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);
	}

	#[sweet_test::test]
	async fn put_and_delete_work() {
		Request::new(format!("{}/put", HTTPBIN))
			.method(HttpMethod::Put)
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);

		Request::new(format!("{}/delete", HTTPBIN))
			.method(HttpMethod::Delete)
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);
	}

	#[sweet_test::test]
	async fn body_raw_works() {
		Request::new(format!("{}/post", HTTPBIN))
			.method(HttpMethod::Post)
			.body_raw(b"rawbytes".to_vec())
			.fetch()
			.await
			.unwrap()
			.xmap(|res| res.status_code())
			.xpect()
			.to_be(200);
	}

	#[test]
	fn invalid_header_fails() {
		let err = Request::new("http://localhost").header("bad\nheader", "val");
		expect(err.is_err()).to_be(true);
	}
}
