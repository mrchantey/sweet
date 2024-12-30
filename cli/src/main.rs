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
	TestWasm(TestWasm),
	BenchAssert(BenchAssert),
}

fn main() -> Result<()> {
	match &Cli::parse().command {
		Commands::TestWasm(cmd) => cmd.run(),
		Commands::BenchAssert(cmd) => cmd.run(),
	}
}
