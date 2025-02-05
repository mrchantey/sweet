#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use sweet::prelude::*;
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
	Rsx(RsxCommand),
	TestServer(TestServer),
	TestWasm(TestWasm),
	Serve(Server),
	Watch(FsWatcher),
}

#[tokio::main]
async fn main() -> Result<()> {
	match Cli::parse().command {
		Commands::BenchAssert(cmd) => cmd.run(),
		Commands::Rsx(cmd) => cmd.run(),
		Commands::TestServer(cmd) => cmd.run(),
		Commands::TestWasm(cmd) => cmd.run(),
		Commands::Serve(cmd) => cmd.run().await,
		Commands::Watch(cmd) => cmd.watch_log().await,
	}
}
