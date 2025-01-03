use super::*;
use std::fmt::Debug;

impl<T> Matcher<T>
where
	T: Debug + Clone,
{
	pub fn to_be<T2: Debug>(&self, other: T2)
	where
		T: PartialEq<T2>,
	{
		self.assert_equal(&other)
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn equality() {
		expect(true).to_be(true);
		expect(true).to_be(true);
		expect(true).not().to_be(false);
	}
}
