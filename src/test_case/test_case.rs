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
		result.map_err(|error| {
			let location = test_err_location(
				&self
					.path()
					.file_stem()
					.unwrap_or_default()
					.to_string_lossy(),
				self.name(),
			);

			let val = format!("{}\n\n{}", location, error.to_string());
			// Temporary fix to avoid assertion failed: psize <= size + max_overhead
			#[cfg(target_arch = "wasm32")]
			{
				crate::wasm_runner::log_web(&val);
				anyhow::anyhow!("")
			}
			#[cfg(not(target_arch = "wasm32"))]
			{
				anyhow::anyhow!(val)
			}
		})
	}

	async fn run_func(&self) -> Result<()>;

	async fn run(&self) -> Result<()> {
		let result = self.run_func().await;
		self.format_error(result)
	}
}


pub fn test_err_full_format(
	location: &str,
	err: &str,
	backtrace: &str,
) -> String {
	format!("{}\n\n{}\n\n{}", location, err, backtrace)
}


/// for a given error `it failed!` format like so:
///
/// ```
/// ● file_name.rs > test_name
///
///
/// it failed!
/// ```
///
pub fn test_err_location(file_stem: &str, test_name: &str) -> String {
	format!("\n● {} > {}", file_stem, test_name).red().bold()
}

/// for a given error `it failed!` format like so:
///
/// ```
/// it failed!
///
/// at path/to/file_name.rs:1:2
/// ```
pub fn test_err_link(file: &str, line: usize, col: usize) -> String {
	let prefix = String::from("at").faint();
	let file_loc = String::from(file).cyan();
	let line_loc = String::from(format!(":{}:{}", line, col)).faint();

	format!("\n{} {}{}\n", prefix, file_loc, line_loc)
}
