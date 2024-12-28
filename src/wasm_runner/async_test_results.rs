use super::AsyncTestDescriptions;
use super::GlobalStore;
use super::SerdeTestDesc;
use anyhow::Result;




#[derive(Debug, Default, Clone)]
pub struct AsyncTestPanics;

impl GlobalStore for AsyncTestPanics {
	fn var_name() -> &'static str { "__sweet_pending_test_results" }
}


impl AsyncTestPanics {
	pub fn set(id: usize, error: String) -> Result<()> {
		Self::set_serde(id, &error)
	}

	pub fn get(id: usize) -> Result<String> { Self::get_serde(id) }


	pub fn collect() -> Result<Vec<(SerdeTestDesc, String)>> {
		Self::collect_serde::<String>()?
			.into_iter()
			.map(|(id, error)| {
				let id: usize = id.parse().unwrap();
				AsyncTestDescriptions::get(id).map(|desc| (desc, error))
			})
			.collect()
		// Ok(Default::default())
	}
}
