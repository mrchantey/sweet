use anyhow::Result;
use clap::Parser;
use std::process::Child;
use std::process::Command;

/// Spin up a server, run some tests, then shut it down.
///
#[derive(Debug, Parser)]
pub struct TestServer {
	/// Blocking command to build the server, ie `cargo build`
	#[arg(long)]
	build_server: String,
	/// Non-blocking command to run the server, ie `cargo run --example test_server`
	#[arg(long)]
	run_server: String,
	/// The test command, ie `cargo test --test integration`
	#[arg(long)]
	run_test: String,
	/// How long to wait in between running the server and running the tests
	#[arg(short, long)]
	delay_secs: Option<f32>,
}



impl TestServer {
	pub fn run(self) -> Result<()> {
		self.build_server()?;
		let mut server = self.run_server()?;
		let result = self.run_test();
		server.kill()?;
		result
	}
	pub fn run_server(&self) -> Result<Child> {
		let child = whitespace_command(&self.run_server).spawn()?;
		Ok(child)
	}

	pub fn build_server(&self) -> Result<()> {
		let cmd = whitespace_command(&self.build_server);
		unwrap_status(cmd)
	}

	pub fn run_test(&self) -> Result<()> {
		if let Some(delay) = self.delay_secs {
			std::thread::sleep(std::time::Duration::from_secs_f32(delay));
		}
		let cmd = whitespace_command(&self.run_test);
		unwrap_status(cmd)
	}
}


fn whitespace_command(cmd: &str) -> Command {
	let cmd = cmd.split_whitespace();
	let mut command = Command::new(cmd.clone().next().unwrap());
	for arg in cmd.skip(1) {
		command.arg(arg);
	}
	command
}


fn unwrap_status(mut cmd: Command) -> Result<()> {
	let status = cmd.status()?;
	if !status.success() {
		Err(anyhow::anyhow!("Command failed: {:?}", status))?;
	}
	Ok(())
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use std::time::Instant;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let delay = 0.5;
		let start = Instant::now();
		TestServer {
			build_server: "echo 'building server'".into(),
			run_server: "echo 'running server'".into(),
			run_test: "echo 'running tests'".into(),
			delay_secs: Some(delay),
		}
		.run()
		.unwrap();
		expect(start.elapsed().as_secs_f32()).to_be_greater_than(delay);
	}
}
