use super::*;
use crate::prelude::*;
use sweet_utils::utils::*;





impl<'a> super::Request<'a> {
	pub async fn fetch(self) -> Result<Response> {
		create_request(&self.url, self.method)?
			.headers(self.headers)
			.xmap(|mut req| {
				if let Some(body) = self.body {
					req = req.body(body.clone());
				}
				if let Some(timeout) = self.timeout {
					req = req.timeout(timeout);
				}
				req
			})
			.send()
			.await
			.map_err(|e| Error::NetworkError(e.to_string()))?
			.xmap(|res| Response::new(res))
			.xok()
	}
}

fn create_request(
	url: &str,
	method: HttpMethod,
) -> Result<reqwest::RequestBuilder> {
	let client = ReqwestClient::client();
	match method {
		HttpMethod::Get => client.get(url).xok(),
		HttpMethod::Post => client.post(url).xok(),
		HttpMethod::Put => client.put(url).xok(),
		HttpMethod::Delete => client.delete(url).xok(),
		HttpMethod::Patch => client.patch(url).xok(),
		HttpMethod::Head => client.head(url).xok(),
		_ => Err(Error::Serialization(format!(
			"Unsupported HTTP method: {:?}",
			method
		))),
	}
}


impl ResponseInner for reqwest::Response {
	fn status_code(&self) -> StatusCode { self.status() }
}
