use anyhow::Result;

//TODO should probably be in matcher module

/// The base struct for all matchers.
pub struct Matcher<T> {
	pub value: T,
	pub negated: bool,
}


impl<T> Matcher<T> {
	/// Create a new Matcher, usually this is done via [`expect`](crate::expect).
	pub(crate) fn new(value: T) -> Matcher<T> {
		Matcher {
			value,
			negated: false,
		}
	}

	/// Map the value of this matcher to a new matcher with the mapped value.
	pub fn map<T2>(&self, func: impl FnOnce(&T) -> T2) -> Matcher<T2> {
		Matcher::new(func(&self.value))
	}
	/// Some assertions do not support negation, in that case call this function within the matcher.
	///
	/// This will return an error if the matcher is already negated.
	pub fn disallow_negated(&self) -> Result<(), String> {
		if self.negated {
			Err("Unsupported: Negation not supported for this matcher, please remove `.not()`".to_string())
		} else {
			Ok(())
		}
	}

	/// Negate this matcher to flip the result of an assertion.
	/// ```rust
	/// # use sweet::prelude::*;
	/// expect(true).not().to_be_false()?;
	/// # Ok::<(), anyhow::Error>(())
	/// ```
	pub fn not(&mut self) -> &mut Self {
		self.negated = true;
		self
	}

	/// Parse a boolean as-is if not negated, otherwise flip it.
	pub fn is_true_with_negated(&self, received: bool) -> bool {
		if self.negated {
			!received
		} else {
			received
		}
	}
}

pub trait MatcherTrait<T> {
	fn get_matcher(&self) -> &Matcher<T>;
	fn get_matcher_mut(&mut self) -> &mut Matcher<T>;
	fn get_matcher_owned(self) -> Matcher<T>;
	fn get_value(&self) -> &T;
	fn get_value_mut(&mut self) -> &mut T;
	fn get_value_owned(self) -> T;
	fn get_negated(&self) -> bool;
	fn set_negated(&mut self, value: bool);
}
impl<T> MatcherTrait<T> for Matcher<T> {
	fn get_matcher_owned(self) -> Matcher<T> { self }
	fn get_matcher(&self) -> &Matcher<T> { self }
	fn get_matcher_mut(&mut self) -> &mut Matcher<T> { self }
	fn get_value(&self) -> &T { &self.value }
	fn get_value_mut(&mut self) -> &mut T { &mut self.value }
	fn get_value_owned(self) -> T { self.value }
	fn get_negated(&self) -> bool { self.negated }
	fn set_negated(&mut self, value: bool) { self.negated = value; }
}
