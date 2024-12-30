use anyhow::Result;
use clap::ArgMatches;
use glob::Pattern;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunnerConfig {
	pub watch: bool,
	/// native only
	pub parallel: bool,
	pub silent: bool,
	#[serde(skip)]
	// these are only used for starting tests running
	// so we dont need to serialize for wasm async
	pub matches: Vec<Pattern>,
}

impl Default for TestRunnerConfig {
	fn default() -> Self {
		Self {
			watch: false,
			parallel: false,
			silent: false,
			matches: Vec::new(),
		}
	}
}

impl TestRunnerConfig {
	pub fn suite_passes_filter(&self, path: &PathBuf) -> bool {
		let matchable_path = path.to_string_lossy();
		self.matches.len() == 0
			|| self.matches.iter().any(|a| a.matches(&matchable_path))
	}
	///Errors on malformed glob pattern.
	pub fn from_arg_matchers(value: &ArgMatches) -> Result<Self> {
		let watch = value.get_flag("watch");
		let parallel = value.get_flag("parallel");
		let silent = value.get_flag("silent");
		let matches = value
			.get_many::<String>("match")
			.unwrap_or_default()
			.map(|s| Pattern::new(&format!("*{s}*")))
			.collect::<Result<Vec<_>, _>>()?;
		Ok(Self {
			watch,
			parallel,
			matches,
			silent,
		})
	}
	pub fn from_env_args() -> Result<Self> {
		let args = std::env::args();
		Self::from_raw_args(args.skip(1))
	}

	pub fn from_raw_args(args: impl Iterator<Item = String>) -> Result<Self> {
		let mut watch = false;
		let mut parallel = false;
		let mut silent = false;
		let mut matches = Vec::new();

		// first arg is executable
		for arg in args.into_iter() {
			match arg.as_str() {
				"--watch" => watch = true,
				"--parallel" => parallel = true,
				"--silent" => silent = true,
				other => {
					if other.starts_with("--") {
						return Err(anyhow::anyhow!("Unknown flag: {}", other));
					}
					matches.push(Pattern::new(&format!("*{}*", other))?);
				}
			}
		}

		Ok(Self {
			watch,
			parallel,
			matches,
			silent,
		})
	}
}

impl std::fmt::Display for TestRunnerConfig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut out = String::new();
		let matches = self
			.matches
			.iter()
			.map(|m| m.to_string())
			.collect::<Vec<_>>()
			.join(" ");
		if self.watch {
			out += format!("watch: true\n").as_str();
		}
		if self.parallel {
			out += format!("parallel: true\n").as_str();
		}
		if self.matches.len() > 0 {
			out += format!("matching: {matches}\n").as_str();
		}
		if self.silent {
			out += format!("silent: true\n").as_str();
		}
		write!(f, "{}", out)
	}
}
