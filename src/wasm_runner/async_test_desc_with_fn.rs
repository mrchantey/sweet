use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use test::TestDesc;

thread_local! {
	static STORE: Rc<RefCell<Vec<AsyncTestDescWithFn>>> = Default::default();
}

pub struct AsyncTestDescWithFn {
	pub desc: TestDesc,
	pub func: Pin<Box<dyn Future<Output = ()>>>,
	/// this will be fulfilled when the test is done
	pub result: Rc<RefCell<Result<(), String>>>,
}

impl AsyncTestDescWithFn {
	pub fn register(self) {
		STORE.with(|store| {
			store.borrow_mut().push(self);
		});
	}
}
