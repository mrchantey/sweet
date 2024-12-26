use super::*;
use crate::prelude::*;
use anyhow::Result;
use std::fmt::Debug;

impl<T> Matcher<T>
where
	T: PartialEq + Debug + Clone,
{
	pub fn to_be(&self, other: T) -> Result<()> {
		self.assert_equal(other).build_res_mapped()
	}

	pub(crate) fn assert_equal(&self, expected: T) -> BacktraceResult {
		if self.equality(&expected) {
			Ok(())
		} else {
			Err(self.to_error(&expected))
		}
	}

	fn equality(&self, other: &T) -> bool {
		if self.negated {
			self.value != *other
		} else {
			self.value == *other
		}
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn equality() -> Result<()> {
		expect(true).to_be(true)?;
		expect(true).not().to_be(false)?;
		Ok(())
	}
}
