use crate::matchers::*;
use anyhow::Result;
use forky::web::*;

impl<T> Matcher<T> {
	pub async fn poll<T2>(
		&mut self,
		mut func: impl FnMut(&mut Self) -> Result<T2> + 'static,
	) -> Result<T2> {
		poll_ok(|| func(self)).await
	}
}
