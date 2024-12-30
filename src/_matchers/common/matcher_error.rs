use crate::prelude::*;
use anyhow::Result;
use colorize::*;
use std::fmt::Debug;

impl<T> Matcher<T> {
	pub fn assert_option_with_received<T2>(
		&self,
		received: Option<T2>,
	) -> Result<T2, BacktraceError> {
		self.disallow_negated()?;
		match received {
			Some(r) => Ok(r),
			None => Err(self.to_error_with_received(&"Some", &"None")),
		}
	}

	pub fn assert_option_with_received_negatable<T2>(
		&self,
		received: Option<T2>,
	) -> BacktraceResult {
		if self.negated && received.is_some() {
			Err(self.to_error_with_received(&"Some", &"Some"))
		} else if !self.negated && received.is_none() {
			Err(self.to_error_with_received(&"Some", &"None"))
		} else {
			Ok(())
		}
	}

	pub fn assert_correct_with_received<T2: Debug, T3: Debug>(
		&self,
		result: bool,
		expected: &T2,
		received: &T3,
	) -> BacktraceResult {
		if self.is_true_with_negated(result) {
			Ok(())
		} else {
			Err(self.to_error_with_received(expected, received))
		}
	}

	/// Testing use only
	pub fn to_error_with_received<T2: Debug, T3: Debug>(
		&self,
		expected: &T2,
		received: &T3,
	) -> BacktraceError {
		let mut expected = format!("{:?}", expected)
			.trim_matches('"')
			.to_string()
			.green();

		if self.negated {
			expected = format!("{} {}", "NOT".bold().green(), expected);
		}
		let received = format!("{:?}", received)
			.trim_matches('"')
			.to_string()
			.red();


		format!("\nExpected: {expected}\nReceived: {received}").into()
	}
}


impl<T> Matcher<T>
where
	T: Debug,
{
	/// Ensure result is true, and check negated
	pub fn assert_correct<T2: Debug>(
		&self,
		result: bool,
		expected: &T2,
	) -> BacktraceResult {
		if self.is_true_with_negated(result) {
			Ok(())
		} else {
			Err(self.to_error(expected))
		}
	}

	pub fn to_error<T2: Debug>(&self, expected: &T2) -> BacktraceError {
		self.to_error_with_received(expected, &self.value)
	}
}
