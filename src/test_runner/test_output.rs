use crate::libtest::TestDescExt;
use test::TestDesc;

/// The raw output of a test.
/// This does not consider #[should_panic]
#[derive(Debug)]
pub enum TestOutput {
	/// The test ran and did not panic or output an error
	NoError,
	/// The test ran and panicked returned an error.
	Error(String),
	Panic(Box<dyn std::any::Any + Send>),
	/// The test is async and registered a future.
	Async,
	/// The test is ignored.
	Ignore,
}

impl TestOutput {
	pub fn from_result(result: Result<(), String>) -> Self { result.into() }

	/// Converts test output into a result,
	/// considering #[should_panic]
	/// # Panics
	/// if the test is async
	pub fn into_result(self, desc: &TestDesc) -> Option<Result<(), String>> {
		match self {
			TestOutput::Async => {
				panic!("async tests cannot be converted to results")
			}
			TestOutput::Ignore => None,
			TestOutput::NoError => {
				Some(TestDescExt::parse_result(desc, Ok(())))
			}
			TestOutput::Error(err) => {
				Some(TestDescExt::parse_result(desc, Err(err)))
			}
			TestOutput::Panic(panic) => Some(TestDescExt::parse_result(
				desc,
				Err(TestDescExt::panic_full_format(desc, panic)),
			)),
		}
	}
}


impl Into<TestOutput> for Result<(), String> {
	fn into(self) -> TestOutput {
		match self {
			Ok(()) => TestOutput::NoError,
			Err(err) => TestOutput::Error(err),
		}
	}
}
