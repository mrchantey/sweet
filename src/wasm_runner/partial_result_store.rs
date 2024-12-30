use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
	static STORE: Rc<RefCell<PartialResultStore>> = Default::default();
}

/// wasm needs to store the results of sync tests
/// globally until the async ones are done
#[derive(Debug, Default)]
pub struct PartialResultStore {
	pub config: TestRunnerConfig,
	pub logger: RunnerLoggerWasm,
	/// the partially collected suite results, will have async results
	/// appended upon completion.
	pub suite_results: Vec<SuiteResult>,
	pub async_suite_outputs: Vec<SuiteOutput>,
}


impl PartialResultStore {
	pub fn set(self) {
		STORE.with(|store| {
			*store.borrow_mut() = self;
		})
	}
	pub fn take() -> Self {
		STORE.with(|store| std::mem::take(&mut *store.borrow_mut()))
	}
}
