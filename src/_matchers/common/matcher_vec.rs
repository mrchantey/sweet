use super::*;
use crate::prelude::BuildableResult;
use anyhow::Result;
use std::fmt::Debug;

impl<T: Debug> Matcher<&Vec<T>> {
	pub fn to_be_empty(&self) -> Result<()> {
		let result = self.value.is_empty();
		let expected = format!("to be empty");
		self.assert_correct(result, &expected).build_res_mapped()
	}
	pub fn any(&self, func: impl FnMut(&T) -> bool) -> Result<()> {
		let result = self.value.iter().any(func);
		let expected = format!("any to match predicate");
		self.assert_correct(result, &expected).build_res_mapped()
	}
}

impl<T: Debug + PartialEq> Matcher<&Vec<T>> {
	pub fn to_contain_element(&self, other: &T) -> Result<()> {
		let result = self.value.contains(other);
		let expected = format!("to contain {:?}", other);
		self.assert_correct(result, &expected).build_res_mapped()
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn vec() -> Result<()> {
		expect(&vec![1, 2, 3]).to_contain_element(&2)?;
		expect(&vec![1, 2, 3]).not().to_contain_element(&4)?;
		expect(&vec![1, 2, 3]).any(|val| val == &2)?;
		expect(&vec![1, 2, 3]).not().any(|val| val == &4)?;
		Ok(())
	}
}
