use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use sweet::prelude::*;

/**
Welcome to the sweet rsx preprocessor!
this command will create a mirror of each file in the given directory,
but with all rsx! macros split into html, css and rust.
*/
#[derive(Debug, Parser)]
pub struct RsxCommand {
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

impl RsxCommand {
	pub fn run(self) -> Result<()> {
		self.for_each_file()?;
		Ok(())
	}


	fn for_each_file(&self) -> Result<()> {
		for file in ReadDir::files_recursive(&self.path)? {
			println!("{}", file.display());
		}
		Ok(())
	}
}
