use crate::prelude::TestOutput;
use std::cell::RefCell;
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;
use std::rc::Rc;
use test::TestDesc;

type Fut = Pin<Box<dyn UnwindSafe + Future<Output = ()>>>;
type FutCell = Rc<RefCell<Option<Fut>>>;

crate::scoped_thread_local! {
	static FUTURE: FutCell
}

thread_local! {
	static ASYNC_TESTS: Rc<RefCell<Vec<(TestDesc,Fut)>>> = Default::default();
}


pub struct SweetTestCollector;


impl SweetTestCollector {
	/// # Panics
	/// If called outside of [`Self::set`]
	pub fn register(fut: Fut) {
		FUTURE.with(|current_fut| {
			*current_fut.borrow_mut() = Some(fut);
		});
	}

	pub fn is_in_scope() -> bool { FUTURE.is_set() }

	/// All tests must be executed within the provided closure.
	/// This ensures sweet::tests can be registered.
	pub fn with_scope<F>(desc: &TestDesc, f: F) -> TestOutput
	where
		F: FnOnce() -> Result<(), String>,
	{
		let val = Rc::new(RefCell::new(None));
		FUTURE.set(&val, || {
			let out = f();
			if let Some(fut) = val.borrow_mut().take() {
				ASYNC_TESTS.with(|async_tests| {
					async_tests.borrow_mut().push((desc.clone(), fut));
				});
				return TestOutput::Async;
			}
			out.into()
		})
	}

	pub fn drain() -> Vec<(TestDesc, Fut)> {
		ASYNC_TESTS
			.with(|async_tests| async_tests.borrow_mut().drain(..).collect())
	}

	// pub fn get() -> Fut {
	// 	CURRENT_FUT.with(|current_fut| {
	// 		let mut current = current_fut.borrow_mut();
	// 		std::mem::replace(&mut *current, Box::pin(async {}))
	// 	})
	// }
}
