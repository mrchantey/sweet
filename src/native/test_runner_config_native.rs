use crate::test_runner::*;
use anyhow::Result;
use clap::ArgMatches;
use glob::Pattern;

impl TestRunnerConfig {
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
		let args = std::env::args().collect();
		Self::from_raw_args(args)
	}

	pub fn from_raw_args(args: Vec<String>) -> Result<Self> {
		let mut watch = false;
		let mut parallel = false;
		let mut silent = false;
		let mut matches = Vec::new();

		// first arg is executable
		for arg in args.iter().skip(1) {
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

// pub fn from_cli_args() -> Self {
// 	fn vec_contains_str(path: &str, args: &Vec<String>) -> bool {
// 		args.iter().any(|a| a == path)
// 	}
// 	fn arr_contains_str(path: &str, arr: &[&str]) -> bool {
// 		arr.iter().any(|a| *a == path)
// 	}

// 	const FLAGS: &'static [&str] = &["-w"];
// 	let mut args = forky::fs::cli_args::get();
// 	let watch = vec_contains_str("-w", &args);
// 	args.retain(|v| !arr_contains_str(v, FLAGS));
// 	Self {
// 		watch,
// 		// parallel: false,
// 		parallel: true,
// 		files: args,
// 	}
// }
