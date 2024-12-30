use super::*;
use crate::prelude::BuildableResult;
use std::fmt::Debug;
use std::fmt::Display;

impl<T: Debug, E: Debug> Matcher<Result<T, E>> {
	pub fn to_be_ok(&self) -> anyhow::Result<()> {
		let result = self.value.is_ok();
		self.assert_correct(result, &"Ok").build_res_mapped()
	}
	pub fn to_be_err(&self) -> anyhow::Result<()> {
		let result = self.value.is_err();
		self.assert_correct(result, &"Error").build_res_mapped()
	}
}
// TODO T shouldt need to be debug
impl<T: Debug, E: Debug + Display> Matcher<Result<T, E>> {
	pub fn to_be_err_str(&self, value: &str) -> anyhow::Result<()> {
		if let Err(err) = &self.value {
			let result = err.to_string() == value;
			self.assert_correct(result, &value).build_res_mapped()
		} else {
			self.assert_correct_with_received(false, &"Error", &"Ok")
				.build_res_mapped()
		}
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::anyhow;

	#[test]
	fn result() -> Result<()> {
		let ok = || -> anyhow::Result<()> { Ok(()) };
		expect(ok()).to_be_ok()?;
		expect(ok()).not().to_be_err()?;

		let err = || -> anyhow::Result<()> { Err(anyhow!("foo")) };

		expect(err()).to_be_err()?;
		expect(err()).not().to_be_ok()?;

		expect(err()).to_be_err_str("foo")?;
		expect(err()).not().to_be_err_str("foobar")?;
		Ok(())
	}
}
