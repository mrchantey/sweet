use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

/// The wasm test runner
///
/// To use add the following:
///
/// ```toml
///
/// # .cargo/config.toml
///
/// [target.wasm32-unknown-unknown]
///
/// runner = 'sweet test-wasm'
///
/// ```
///
#[derive(Debug, Parser)]
pub struct TestWasm {
	/// the file passed in by cargo test.
	///
	/// It will look something like $CARGO_TARGET_DIR/wasm32-unknown-unknown/debug/deps/hello_test-c3298911e67ad05b.wasm
	test_binary: String,
	/// arguments passed to wasm-bindgen
	#[arg(long)]
	wasm_bindgen_args: Option<String>,

	// we wont actuallly use this because the args will
	// be passed to deno, but it provides --help messages
	#[command(flatten)]
	runner_args: sweet::prelude::TestRunnerConfig,
}


impl TestWasm {
	pub fn run(&self) -> anyhow::Result<()> {
		self.run_wasm_bindgen()?;
		self.init_deno()?;
		self.run_deno()?;
		Ok(())
	}
	fn run_wasm_bindgen(&self) -> Result<()> {
		let status = Command::new("wasm-bindgen")
			.arg("--out-dir")
			.arg(sweet_target_dir())
			.arg("--out-name")
			.arg("bindgen")
			.arg("--target")
			.arg("web")
			.arg("--no-typescript")
			.arg(&self.test_binary)
			.args(
				self.wasm_bindgen_args
					.as_deref()
					.unwrap_or_default()
					.split_whitespace(),
			)
			.status()?;

		if !status.success() {
			anyhow::bail!("wasm-bindgen command failed");
		}
		Ok(())
	}


	/// Move the deno file to the correct directory,
	/// if this is the first time this will also ensure deno is installed
	/// by running `deno --version`
	fn init_deno(&self) -> Result<()> {
		let deno_runner_path = deno_runner_path();
		if let Ok(true) = fs::exists(&deno_runner_path) {
			return Ok(());
		};

		let deno_installed =
			match Command::new("deno").arg("--version").status() {
				Ok(val) => val.success(),
				_ => false,
			};
		if !deno_installed {
			anyhow::bail!(INSTALL_DENO);
		}
		// wasm-bindgen will ensure parent dir exists
		fs::write(deno_runner_path, include_str!("./deno.ts"))?;
		Ok(())
	}

	fn run_deno(&self) -> Result<()> {
		// args will look like this so skip 3
		// sweet test-wasm binary-path *actual-args
		// why doesnt it work with three?
		let args = std::env::args().skip(2).collect::<Vec<_>>();
		let status = Command::new("deno")
			.arg("--allow-read")
			.arg("--allow-net")
			.arg("--allow-env")
			.arg(deno_runner_path())
			.args(args)
			.status()?;

		if !status.success() {
			anyhow::bail!("deno command failed");
		}
		Ok(())
	}
}

fn workspace_root() -> PathBuf {
	let root_str = std::env::var("SWEET_ROOT").unwrap_or_default();
	PathBuf::from_str(&root_str).unwrap()
}

fn sweet_target_dir() -> PathBuf {
	let mut path = workspace_root();
	path.push("target/sweet");
	path
}

fn deno_runner_path() -> PathBuf {
	let mut path = sweet_target_dir();
	path.push("deno.ts");
	path
}

const INSTALL_DENO: &str = "
🦖 Sweet uses Deno for WASM tests 🦖

Installation:
shell: 				curl -fsSL https://deno.land/install.sh | sh
powershell: 	irm https://deno.land/install.ps1 | iex
other: 				https://docs.deno.com/runtime/getting_started/installation/

";


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;
	use test_wasm::deno_runner_path;

	#[test]
	fn works() {
		expect(deno_runner_path().to_string_lossy())
			.to_end_with("target/sweet/deno.ts");
		expect(true).to_be_false();
	}
}
