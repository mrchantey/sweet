use super::GlobalStore;
use super::RunnerLoggerWasm;
use crate::prelude::SuiteResult;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
/// wasm needs to store the results of sync tests
/// globally until the async ones are done
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialResultStore {
	pub logger: RunnerLoggerWasm,
	pub results: Vec<SuiteResult>,
}


impl PartialResultStore {
	pub fn new(logger: RunnerLoggerWasm, results: Vec<SuiteResult>) -> Self {
		Self { logger, results }
	}
	pub fn set(&self) -> Result<()> { Self::set_serde("results", self) }
	pub fn get() -> Result<Self> { Self::get_serde("results") }
}


impl GlobalStore for PartialResultStore {
	fn var_name() -> &'static str { "__sweet_partial_results" }



	// fn store(&self) {
	// 	let store = self;
	// 	let store_str = serde_json::to_string(store).unwrap();
	// }
}
