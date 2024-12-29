use crate::prelude::*;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;


thread_local! {
	static STORE: Rc<RefCell<PartialResultStore>> = Default::default();
}




/// wasm needs to store the results of sync tests
/// globally until the async ones are done
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PartialResultStore {
	pub config: TestRunnerConfig,
	pub logger: RunnerLoggerWasm,
	pub suite_results: Vec<SuiteResult>,
}


impl PartialResultStore {
	pub fn set(&self) {
		STORE.with(|store| {
			*store.borrow_mut() = self.clone();
		})
	}
	pub fn get() -> Result<Self> {
		Ok(STORE.with(|store| store.borrow().clone()))
	}
}
