use crate::prelude::BacktraceFile;
use crate::prelude::TestResult;
use crate::utils::payload_to_string;
use std::cell::RefCell;
// use std::panic::PanicHookInfo;
use std::rc::Rc;
use test::ShouldPanic;
use test::TestDesc;


crate::scoped_thread_local! {
	static DESC: Rc<TestDesc>
}

crate::scoped_thread_local! {
	static PANIC_RESULT: Rc<RefCell<Option<TestResult>>>
}

/// when a test panics, store it globally
/// and retrieve immediately after
pub struct TestCaseContext;


impl TestCaseContext {
	pub fn set_panic_hook() {
		let default_hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info| {
			const FRAME_DEPTH: usize = 7 + 1; // the 1 is for the get_frame func
			if !DESC.is_set() {
				default_hook(info);
			}

			let test_result = match DESC.with(|desc| desc.should_panic) {
				ShouldPanic::Yes => TestResult::Pass,
				ShouldPanic::YesWithMessage(_) => TestResult::Pass,
				ShouldPanic::No => {
					let bt_str = BacktraceFile::backtrace_str(FRAME_DEPTH)
						.unwrap_or_else(|err| err.to_string());
					let payload_str = payload_to_string(info.payload());
					TestResult::Fail(format!("{}\n\n{}", payload_str, bt_str))
				}
			};

			// println!("Panic hook: {:?}", test_result);

			PANIC_RESULT.with(|panic_result| {
				*panic_result.borrow_mut() = Some(test_result);
			})
		}));
	}


	/// Runs a test case with a context that catches panics
	/// and maps them to the correct [TestResult]
	pub fn with_scope<F>(desc: &Rc<TestDesc>, func: F) -> TestResult
	where
		F: FnOnce() -> TestResult,
	{
		let panic_result = Default::default();

		
		let output = PANIC_RESULT.set(&panic_result, || DESC.set(desc, func));

		let mut panic_result = panic_result.borrow_mut();
		match panic_result.take() {
			Some(result) => result,
			None => output,
		}
	}
}
