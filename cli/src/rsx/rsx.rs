use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;



#[derive(Debug,Parser)]
pub struct Rsx{


}



impl Rsx{
	pub fn run(self)->Result<()>{
		Ok(())
	}

}


