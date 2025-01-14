#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use sweet_cli::prelude::*;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	BenchAssert(BenchAssert),
	Rsx(Rsx),
	TestServer(TestServer),
	TestWasm(TestWasm),
}

fn main() -> Result<()> {
	match Cli::parse().command {
		Commands::BenchAssert(cmd) => cmd.run(),
		Commands::Rsx(cmd) => cmd.run(),
		Commands::TestServer(cmd) => cmd.run(),
		Commands::TestWasm(cmd) => cmd.run(),
	}
}
