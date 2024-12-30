use colorize::*;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
// use std::default;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuiteResult {
	pub file: String,
	pub num_tests: usize,
	pub num_ignored: usize,
	pub failed: Vec<String>,
}


impl SuiteResult {
	pub fn new(file: String, tests: usize, skipped: usize) -> Self {
		SuiteResult {
			file,
			num_tests: tests,
			num_ignored: skipped,
			failed: Vec::new(),
		}
	}
	pub fn with_failed(mut self, failed: Vec<String>) -> Self {
		self.failed = failed;
		self
	}

	pub fn in_progress_str(&self) -> String {
		let mut value = " RUNS ".black().bold().yellowb();
		value += " ";
		value += self.pretty_path().as_str();
		value
	}


	pub fn end_str(&self) -> String {
		let mut val = if self.failed.len() == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		val += " ";
		val += self.pretty_path().as_str();

		val += &self
			.failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());

		val
	}

	fn pretty_path(&self) -> String {
		let file = PathBuf::from(&self.file);

		let name = file
			.file_name()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string()
			.bold();
		let dir = file
			.parent()
			.unwrap_or_else(|| Path::new(""))
			.to_string_lossy()
			.to_string()
			.faint();
		let slash = "/".faint();
		format!("{dir}{slash}{name}")
	}
}
