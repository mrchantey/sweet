use super::GlobalStore;
use super::SerdeTestDesc;
use anyhow::Result;
use test::TestDesc;
use test::TestDescAndFn;


const PENDING_ASYNC_PREFIX: &str = "pending_async|";

pub fn panic_with_id(id: usize) {
	panic!("{}{}", PENDING_ASYNC_PREFIX, id);
}


#[derive(Debug, Default, Clone)]
pub struct AsyncTestDescriptions;


impl AsyncTestDescriptions {
	/// If a test panics with a message starting with `pending_async`
	/// this means it is async and we will need to catch its output
	/// after initial run.
	pub fn try_store(test: &TestDescAndFn, err: &str) -> Result<bool> {
		if !err.starts_with(PENDING_ASYNC_PREFIX) {
			return Ok(false);
		}
		let id = err
			.trim_start_matches(PENDING_ASYNC_PREFIX)
			.parse::<usize>()
			.unwrap_or_else(|_| {
				panic!("failed to parse pending async id: {}", err)
			});
		Self::store(id, &test.desc)?;

		Ok(true)
	}


	pub fn store(id: usize, desc: &TestDesc) -> Result<()> {
		let serde_desc: SerdeTestDesc = desc.clone().into();
		let serde = serde_json::to_string(&serde_desc)?;
		Self::set_field(id, serde);
		Ok(())
	}
	pub fn get(id: usize) -> Result<SerdeTestDesc> {
		let serde = Self::get_field(id).map_err(|_| {
			anyhow::anyhow!("no pending test desc for id: {}", id)
		})?;
		let serde: String = serde.as_string().ok_or_else(|| {
			anyhow::anyhow!("pending test desc is not a string")
		})?;
		let desc: SerdeTestDesc = serde_json::from_str(&serde)?;
		Ok(desc)
	}
}


impl GlobalStore for AsyncTestDescriptions {
	fn var_name() -> &'static str { "__sweet_pending_test_descriptions" }
}
