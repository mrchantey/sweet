use super::*;
use crate::prelude::BuildableResult;
use anyhow::Result;

impl<T> Matcher<Option<T>>
where
	T: std::fmt::Debug,
{
	pub fn to_be_option(&self, expected: bool) -> Result<()> {
		if expected {
			let result = self.value.is_some();
			self.assert_correct(result, &"Some").build_res_mapped()
		} else {
			let result = self.value.is_none();
			self.assert_correct(result, &"None").build_res_mapped()
		}
	}
	pub fn to_be_some(&self) -> Result<()> {
		let result = self.value.is_some();
		self.assert_correct(result, &"Some").build_res_mapped()
	}
	pub fn as_some(self) -> Result<Matcher<T>> {
		if let Some(value) = self.value {
			Ok(Matcher::new(value))
		} else {
			Err(self.to_error(&"Some").build_err())
		}
	}
	pub fn to_be_none(&self) -> Result<()> {
		let result = self.value.is_none();
		self.assert_correct(result, &"None").build_res_mapped()
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn option() -> Result<()> {
		expect(Some(true)).to_be_some()?;
		expect(Some(true)).not().to_be_none()?;

		expect(None::<bool>).to_be_none()?;
		expect(None::<bool>).not().to_be_some()?;
		Ok(())
	}
}
