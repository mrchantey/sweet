use super::AsyncTestDescriptions;
use crate::libtest::LibtestHash;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use test::TestDesc;



thread_local! {
	static STORE: Rc<RefCell<HashMap<LibtestHash,Result<(),String>>>> = Default::default();
}



#[derive(Debug, Default, Clone)]
pub struct AsyncTestResults;

impl AsyncTestResults {
	pub fn set(id: LibtestHash, result: Result<(), String>) {
		STORE.with(|store| {
			store.borrow_mut().insert(id, result);
		});
	}

	pub fn get(id: LibtestHash) -> Result<Result<(), String>> {
		STORE.with(|store| {
			store
				.borrow()
				.get(&id)
				.cloned()
				.ok_or_else(|| anyhow::anyhow!("async test not found: {}", id))
		})
	}


	pub fn collect() -> Result<Vec<(TestDesc, Result<(), String>)>> {
		STORE
			.with(|store| store.borrow_mut().drain().collect::<Vec<_>>())
			.into_iter()
			.map(|(id, error)| {
				AsyncTestDescriptions::get(id).map(|desc| (desc, error))
			})
			.collect()
		// Ok(Default::default())
	}
}
