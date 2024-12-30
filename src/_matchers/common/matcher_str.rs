use super::*;
use crate::prelude::BuildableResult;
use anyhow::Result;

impl<T: std::fmt::Debug + AsRef<str>> Matcher<T> {
	pub fn to_contain(&self, other: &str) -> Result<()> {
		let result = self.value.as_ref().contains(other);
		let expected = format!("to contain '{}'", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn to_start_with(&self, other: &str) -> Result<()> {
		let result = self.value.as_ref().starts_with(other);
		let expected = format!("to start with '{}'", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn to_end_with(&self, other: &str) -> Result<()> {
		let result = self.value.as_ref().ends_with(other);
		let expected = format!("to end with '{}'", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn to_be_str(&self, other: &str) -> Result<()> {
		let result = self.value.as_ref() == other;
		let expected = format!("to be '{}'", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
}

// impl<T: std::fmt::Debug + ToString> Matcher<Option<T>> {
// 	pub fn to_be_str(&self, other: &str) -> Result<()> {
// 		if let Some(value) = &self.value {
// 			let result = value.to_string() == other;
// 			let expected = format!("to be '{}'", other);
// 			self.assert_correct(result, &expected)
// 		} else {
// 			let result = false;
// 			let expected = format!("to be '{}'", other);
// 			self.assert_correct(result, &expected)
// 		}
// 	}
// }


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn str() -> Result<()> {
		// expect("foo".to_string()).to_be("foo")?;

		expect("foobar").to_contain("bar")?;
		expect("foobar").not().to_contain("baz")?;

		expect("foobar").to_start_with("foo")?;
		expect("foobar").not().to_start_with("bar")?;

		expect("foobar").to_end_with("bar")?;
		expect("foobar").not().to_end_with("foo")?;
		Ok(())
	}
}
