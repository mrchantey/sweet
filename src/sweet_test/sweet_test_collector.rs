use std::cell::RefCell;
use std::collections::HashSet;
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;
use std::rc::Rc;
use test::TestDesc;

type Fut = Pin<Box<dyn UnwindSafe + Future<Output = ()>>>;
type FutCell = Rc<RefCell<Option<Fut>>>;

crate::scoped_thread_local! {
	static CURRENT_FUT: FutCell
}

thread_local! {
	/// Files that contain async tests should not emit PASS before all async tests have completed
	static ASYNC_TEST_FILES: Rc<RefCell<HashSet<&'static str>>> = Default::default();
	static ASYNC_TESTS: Rc<RefCell<Vec<(TestDesc,Fut)>>> = Default::default();
}


pub struct SweetTestCollector;


impl SweetTestCollector {
	/// # Panics
	/// If called outside of [`Self::set`]
	pub fn register(fut: Fut) {
		CURRENT_FUT.with(|current_fut| {
			*current_fut.borrow_mut() = Some(fut);
		});
	}


	pub fn contains_async_test(source_file: &str) -> bool {
		ASYNC_TEST_FILES.with(|files| files.borrow().contains(source_file))
	}

	/// All tests must be executed within the provided closure.
	/// This ensures sweet::tests can be registered.
	pub fn with_scope<F, R>(desc: &TestDesc, f: F) -> R
	where
		F: FnOnce() -> R,
	{
		let val = Rc::new(RefCell::new(None));
		CURRENT_FUT.set(&val, || {
			let out = f();
			if let Some(fut) = val.borrow_mut().take() {
				ASYNC_TEST_FILES.with(|files| {
					files.borrow_mut().insert(desc.source_file);
				});
				ASYNC_TESTS.with(|async_tests| {
					async_tests.borrow_mut().push((desc.clone(), fut));
				});
			}
			out
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
