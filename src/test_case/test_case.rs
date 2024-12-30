use super::*;
use anyhow::Result;
use colorize::*;
use std::panic::RefUnwindSafe;
use std::path::PathBuf;

// #[deprecated = "use run_test"]
pub trait TestCase
where
	Self: RefUnwindSafe,
{
	fn path(&self) -> PathBuf;
	fn name(&self) -> &str;
	fn config(&self) -> &TestCaseConfig;
	fn format_error<E: ToString>(
		&self,
		result: Result<(), E>,
	) -> anyhow::Result<()> {
		unreachable!()
		// result.map_err(|error| {
		// 	let location = test_err_location(
		// 		&self
		// 			.path()
		// 			.file_stem()
		// 			.unwrap_or_default()
		// 			.to_string_lossy(),
		// 		self.name(),
		// 	);

		// 	let val = format!("{}\n\n{}", location, error.to_string());
		// 	// Temporary fix to avoid assertion failed: psize <= size + max_overhead
		// 	#[cfg(target_arch = "wasm32")]
		// 	{
		// 		crate::prelude::log_val(&val);
		// 		anyhow::anyhow!("")
		// 	}
		// 	#[cfg(not(target_arch = "wasm32"))]
		// 	{
		// 		anyhow::anyhow!(val)
		// 	}
		// })
	}

	async fn run_func(&self) -> Result<()>;

	async fn run(&self) -> Result<()> {
		let result = self.run_func().await;
		self.format_error(result)
	}
}
