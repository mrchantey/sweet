use super::*;
use crate::prelude::BuildableResult;
use anyhow::Result;

impl<T> Matcher<T>
where
	T: PartialOrd + std::fmt::Debug + std::marker::Copy,
{
	pub fn to_be_less_than(&self, other: T) -> Result<()> {
		let result = self.value < other;
		let expected = format!("less than {:?}", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn to_be_less_or_equal_to(&self, other: T) -> Result<()> {
		let result = self.value <= other;
		let expected = format!("less or equal to {:?}", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn to_be_greater_than(&self, other: T) -> Result<()> {
		let result = self.value > other;
		let expected = format!("greater than {:?}", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn to_be_greater_or_equal_to(&self, other: T) -> Result<()> {
		let result = self.value >= other;
		let expected = format!("greater or equal to {:?}", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;


	#[test]
	fn order() -> Result<()> {
		expect(0).to_be_greater_or_equal_to(0)?;
		expect(10).to_be_greater_than(-10)?;
		expect(10).not().to_be_greater_than(11)?;
		Ok(())
	}
}
