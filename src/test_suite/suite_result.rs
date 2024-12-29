use colorize::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use test::TestDesc;
// use std::default;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuiteResult {
	pub file: String,
	pub tests: usize,
	pub skipped: usize,
	pub failed: Vec<String>,
}


impl SuiteResult {
	pub fn new(file: String, tests: usize, skipped: usize) -> Self {
		SuiteResult {
			file,
			tests,
			skipped,
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

	pub fn append_sweet_tests(
		suite_results: &mut Vec<Self>,
		results: Vec<(TestDesc, Option<String>)>,
	) {
		let failures = results.into_iter().filter_map(|tuple| match tuple.1 {
			Some(failure) => Some((tuple.0, failure)),
			None => None,
		});

		let mut async_suites = HashSet::new();

		for (desc, failure) in failures {
			let suite_result = suite_results
				.iter_mut()
				.find(|suite| suite.file == desc.source_file)
				.unwrap();
			suite_result.failed.push(failure);
			async_suites.insert(suite_result.file.clone());
		}

		// these never got a chance to print their error
		for suite in suite_results.iter_mut() {
			if async_suites.contains(&suite.file) {
				crate::log!("{}", suite.end_str());
			}
		}
	}
}
