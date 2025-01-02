use crate::prelude::*;
use std::panic::PanicHookInfo;
use test::ShouldPanic;
use test::TestDesc;


/// a method for sending test descriptions with outputs
/// This implementation may change to be more restricted
/// to reduce clone cost
pub struct TestDescAndResult {
	pub desc: TestDesc,
	pub result: TestResult,
}


pub struct TestDescAndFuture {
	pub desc: TestDesc,
	pub fut: SweetFutFunc,
}

impl TestDescAndFuture {
	pub fn new(desc: TestDesc, fut: SweetFutFunc) -> Self { Self { desc, fut } }
}

impl TestDescAndResult {
	pub fn new(desc: TestDesc, result: TestResult) -> Self {
		Self { desc, result }
	}
	pub fn ignore(desc: TestDesc, ignore_msg: &'static str) -> Self {
		Self::new(desc, TestResult::Ignore(ignore_msg))
	}
}

pub enum TestResultOrFut {
	Result(TestResult),
	Fut(SweetFutFunc),
}

impl Into<TestResultOrFut> for TestResult {
	fn into(self) -> TestResultOrFut { TestResultOrFut::Result(self) }
}

/// The raw output of a test.
/// This does not consider #[should_panic]
pub enum TestOutput {
	/// The test ran and did not panic or output an error
	Ok,
	/// The test ran and panicked returned an error.
	Error(String),
	Panic(String),
	Ignored(&'static str),
}

#[derive(Debug)]
pub enum TestResult {
	Pass,
	Fail(String),
	Ignore(&'static str),
}

impl TestResult {
	pub fn status_str(&self) -> &'static str {
		match self {
			TestResult::Pass => "PASS",
			TestResult::Fail(_) => "FAIL",
			TestResult::Ignore(_) => "SKIP",
		}
	}

	pub fn stdout(&self) -> String {
		match self {
			TestResult::Pass => "".to_string(),
			TestResult::Fail(msg) => format!("\n{}", msg),
			TestResult::Ignore(msg) => format!("\n{}", msg),
		}
	}


	/// This must be called directly from the panic hook
	/// or else the bactrace frame will be off
	pub fn from_panic(info: &PanicHookInfo, desc: &TestDesc) -> Self {
		const FRAME_DEPTH: usize = 7 + 2;
		match &desc.should_panic {
			ShouldPanic::Yes => TestResult::Pass,
			ShouldPanic::YesWithMessage(_) => TestResult::Pass,
			ShouldPanic::No => {
				let bt_str = BacktraceFile::backtrace_str(FRAME_DEPTH)
					.unwrap_or_else(|err| err.to_string());
				let payload_str = payload_to_string(info.payload());
				TestResult::Fail(Self::format_backtrace(payload_str, bt_str))
			}
		}
	}


	pub fn from_test_result(res: Result<(), String>, desc: &TestDesc) -> Self {
		let parsed_result = match (res, &desc.should_panic) {
			(Ok(()), ShouldPanic::No) => Ok(()),
			(Ok(()), ShouldPanic::Yes) => Err("Expected panic".to_string()),
			(Ok(()), ShouldPanic::YesWithMessage(msg)) => {
				Err(format!("Expected panic: {}", msg))
			}
			(Err(err), ShouldPanic::Yes) => {
				Err(format!("Expected panic, received error: {}", err))
			}
			(Err(err), ShouldPanic::YesWithMessage(msg)) => Err(format!(
				"Expected panic '{}', received error: {}",
				msg, err
			)),
			(Err(err), ShouldPanic::No) => Err(err),
		};

		match parsed_result {
			Ok(()) => TestResult::Pass,
			Err(err) => TestResult::Fail(Self::format_backtrace(
				err,
				BacktraceFile::file_context_from_desc(desc).unwrap_or_default(),
			)),
		}
	}

	/// We ignore the catch unwind because we use the panic store
	pub fn flatten_catch_unwind(
		res: Result<Result<(), String>, Box<dyn std::any::Any + Send>>,
	) -> Self {
		match res {
			Ok(Ok(())) => Self::Pass,
			Ok(Err(err)) => Self::Fail(err),
			Err(_) => Self::Pass,
		}
	}

	/// Panics are caught in the hook for backtracing
	/// so we discard the catch_unwind
	pub fn catch_unwind_test_fn(
		func: fn() -> Result<(), String>,
	) -> Option<Self> {
		match std::panic::catch_unwind(func) {
			Ok(Ok(())) => Some(Self::Pass),
			Ok(Err(err)) => Some(Self::Fail(err)),
			Err(_) => None,
		}
	}

	fn format_backtrace(err: String, bt: String) -> String {
		format!("{}\n\n{}", err, bt)
	}



	pub fn pass() -> Self { Self::Pass }
	pub fn fail(msg: impl Into<String>) -> Self { Self::Fail(msg.into()) }
	pub fn ignored(msg: &'static str) -> Self { Self::Ignore(msg) }
}


impl std::fmt::Display for TestResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TestResult::Pass => write!(f, "Pass"),
			TestResult::Fail(msg) => write!(f, "Fail: {}", msg),
			TestResult::Ignore(msg) => write!(f, "Ignore: {}", msg),
		}
	}
}
