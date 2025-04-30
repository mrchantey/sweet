use super::*;
use http::StatusCode;

#[derive(Debug)]
pub struct Response {
	#[cfg(not(target_arch = "wasm32"))]
	pub inner: reqwest::Response,
}

impl Response {
	#[cfg(not(target_arch = "wasm32"))]
	pub fn new(inner: reqwest::Response) -> Self { Self { inner } }

	/// Becomes an error if the response is not 2xx
	pub fn into_result(self) -> Result<Self> {
		if self.status_code().is_success() {
			Ok(self)
		} else {
			Err(Error::ResponseNotOk(self.status_code()))
		}
	}
}

impl ResponseInner for Response {
	fn status_code(&self) -> StatusCode { self.inner.status() }
}


pub trait ResponseInner {
	fn status_code(&self) -> StatusCode;
}
