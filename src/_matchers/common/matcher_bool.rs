use super::*;
use crate::prelude::*;
use anyhow::Result;

impl Matcher<bool> {
	pub fn to_be_true(&self) -> Result<()> {
		self.assert_equal(true).build_res_mapped()
	}
	pub fn to_be_false(&self) -> Result<()> {
		self.assert_equal(false).build_res_mapped()
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn bool() -> Result<()> {
		expect(true).to_be_true()?;
		expect(false).not().to_be_true()?;

		expect(false).to_be_false()?;
		expect(true).not().to_be_false()?;
		Ok(())
	}
}
