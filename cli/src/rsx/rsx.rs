use anyhow::Result;
use clap::Parser;
use forky::prelude::fs::read_dir_recursive;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;



#[derive(Debug, Parser)]
pub struct Rsx {
	// The path to search
	path: PathBuf,
	/// Output to stdout instead of out dir
	#[arg(long)]
	dry: bool,
	/// Output directory
	#[arg(short, long, default_value = "./rsx")]
	// 1000 is the most gracious
	out_dir: String,
}

impl Rsx {
	pub fn run(self) -> Result<()> { Ok(()) }


	fn for_each_file(&self) -> Result<()> {
		for file in read_dir_recursive(&self.path) {}
		Ok(())
	}
}
