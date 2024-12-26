use crate::prelude::*;
use anyhow::Result;
use std::fmt::Debug;

impl<I, O, F> Matcher<&MockFunc<I, O, F>> {
	pub fn to_have_been_called(&self) -> Result<()> {
		let received = self.value.called.lock().unwrap().len();
		self.assert_correct_with_received(
			received > 0,
			&"to have been called",
			&false,
		)
		.build_res_mapped()
	}
	pub fn to_have_been_called_times(&self, times: usize) -> Result<()> {
		let received = self.value.called.lock().unwrap().len();
		self.assert_correct_with_received(
			received == times,
			&format!("to have been called {times} times"),
			&format!("called {received} times"),
		)
		.build_res_mapped()
	}
}
impl<I, O: Clone, F> Matcher<&MockFunc<I, O, F>> {
	pub fn nth_return(&self, time: usize) -> Result<Matcher<O>> {
		let vec = self.value.called.lock().unwrap();
		if let Some(received) = vec.get(time) {
			Ok(Matcher::new(received.clone()))
		} else {
			Err(self
				.to_error_with_received(&"to have been called", &false)
				.build_err())
		}
	}
}
impl<I, O: Debug + PartialEq, F> Matcher<&MockFunc<I, O, F>> {
	/// checks the first time it was called
	pub fn to_have_returned_with(&self, expected: &O) -> Result<()> {
		if let Some(received) = self.value.called.lock().unwrap().first() {
			self.assert_correct_with_received(
				received == expected,
				expected,
				received,
			)
			.build_res_mapped()
		} else {
			Err(self
				.to_error_with_received(&"to have been called", &false)
				.build_err())
		}
	}
	pub fn to_have_returned_nth_with(
		&self,
		time: usize,
		expected: &O,
	) -> Result<()> {
		if let Some(received) = self.value.called.lock().unwrap().get(time) {
			self.assert_correct_with_received(
				received == expected,
				expected,
				received,
			)
			.build_res_mapped()
		} else {
			self.to_error_with_received(&"to have been called", &false)
				.build_res()
		}
	}
}

//TODO to_have_been_called_with



#[cfg(test)]
mod test {
	use crate::prelude::*;
	#[test]
	fn test_mock_trigger() -> Result<()> {
		let func = mock_trigger();
		func.call(());
		func.call(());
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_been_called_times(2)?;
		expect(&func.clone()).not().to_have_been_called_times(1)?;
		Ok(())
	}
	#[test]
	fn test_mock_func() -> Result<()> {
		let func = mock_func(|i| i * 2);
		func.call(0);
		func.call(2);
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_returned_with(&0)?;
		expect(&func).not().to_have_returned_with(&4)?;
		expect(&func).nth_return(1)?.to_be(4)?;
		expect(&func).nth_return(0)?.to_be(0)?;
		expect(&func).nth_return(1)?.to_be(4)?;
		Ok(())
	}
}
