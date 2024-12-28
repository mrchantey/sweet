use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
	static FOO: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
}

fn main() {
	FOO.with(|f| {
		*f.borrow_mut() += 20;
	});
	sweet::log!("val: {:?}", FOO.with(|f| *f.borrow()));
}
