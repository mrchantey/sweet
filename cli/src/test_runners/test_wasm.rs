use anyhow::Result;
use clap::Parser;
use std::fs;
use std::process::Command;

const DENO_RUNNER_PATH: &str = "./target/sweet/deno.ts";


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
		self.create_deno_runner()?;
		self.run_deno()?;
		Ok(())
	}
	fn run_wasm_bindgen(&self) -> Result<()> {
		let status = Command::new("wasm-bindgen")
			.arg("--out-dir")
			.arg("./target/sweet")
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

	fn create_deno_runner(&self) -> Result<()> {
		if let Ok(true) = fs::exists(DENO_RUNNER_PATH) {
			return Ok(());
		}
		// wasm-bindgen will ensure parent dir exists
		fs::write(DENO_RUNNER_PATH, include_str!("./deno.ts"))?;
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
			.arg(DENO_RUNNER_PATH)
			.args(args)
			.status()?;

		if !status.success() {
			anyhow::bail!("deno command failed");
		}
		Ok(())
	}
}
