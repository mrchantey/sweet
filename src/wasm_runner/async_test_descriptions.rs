use crate::prelude::*;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use test::TestDesc;
use test::TestDescAndFn;


thread_local! {
	static STORE: Rc<RefCell<HashMap<LibtestHash,TestDesc>>> = Default::default();
}

#[derive(Debug, Default, Clone)]
pub struct AsyncTestDescriptions;

impl AsyncTestDescriptions {
	/// If a test panics with a message starting with `pending_async`
	/// this means it is async and we will need to catch its output
	/// after initial run.
	pub fn store(test: &TestDescAndFn) {
		Self::set(TestDescExt::hash(&test.desc), &test.desc);
	}

	fn set(id: LibtestHash, desc: &TestDesc) {
		STORE.with(|store| {
			store.borrow_mut().insert(id, desc.clone());
		});
	}
	pub fn get(id: LibtestHash) -> Result<TestDesc> {
		STORE.with(|store| {
			store
				.borrow()
				.get(&id)
				.cloned()
				.ok_or_else(|| anyhow::anyhow!("async test not found: {}", id))
		})
	}
}
