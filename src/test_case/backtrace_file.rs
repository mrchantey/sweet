use crate::libtest::test_err_link;
use backtrace::BacktraceSymbol;
use colorize::*;
use forky::prelude::*;
use std::fs;
use std::io;
use std::path::PathBuf;

const LINE_CONTEXT_SIZE: usize = 2;

pub struct BacktraceFile {
	pub file: PathBuf,
	pub file_rel: PathBuf,
	pub line: u32,
	pub col: u32,
}

impl BacktraceFile {
	/// This method is hardcoded to only be called from Matcher::new

	pub fn new(symbol: &BacktraceSymbol) -> BacktraceFile {
		let file = symbol.filename().expect("invalid backtrace symbol");
		let file_rel = file.relative().unwrap_or(file);
		let line = symbol.lineno().unwrap_or(0);
		if let Some(col) = symbol.colno() {
			BacktraceFile {
				file: file.to_path_buf(),
				file_rel: file_rel.to_path_buf(),
				line,
				col,
			}
		} else {
			BacktraceFile {
				file: file.to_path_buf(),
				file_rel: file_rel.to_path_buf(),
				line,
				col: 0,
			}
		}
	}


	/// # Errors
	/// This function will return an error if the file cannot be read
	pub fn file_context(&self) -> io::Result<String> {
		//line number is one-indexed
		let line_no = self.line as usize;
		let col_no = self.col as usize;
		let lines = fs::read_to_string(&self.file)?;
		let lines: Vec<&str> = lines.split("\n").collect();
		let start =
			usize::max(0, line_no.saturating_sub(LINE_CONTEXT_SIZE - 1));
		let end = usize::min(lines.len() - 1, line_no + LINE_CONTEXT_SIZE);

		let mut output = String::new();

		for i in start..end {
			let curr_line_no = i + 1;
			let is_err_line = curr_line_no == line_no;
			let prefix =
				String::from(if is_err_line { ">" } else { " " }).red();

			let buffer = line_number_buffer(curr_line_no);
			let line_prefix =
				String::from(format!("{}{}|", curr_line_no, buffer)).faint();
			let full_prefix = format!("{} {}", prefix, line_prefix);
			// let prefix_len = 6;
			output.push_string(&full_prefix);
			output.push_str(lines[i]);
			output.push('\n');
			if is_err_line {
				//TODO string length
				output.push_string(
					&format!("{}|", " ".repeat(2 + LINE_BUFFER_LEN)).faint(),
				);
				output.push_str(&" ".repeat(col_no));
				output.push_str_line(String::from("^").red().as_str());
			}
		}

		output.push_string(&test_err_link(
			self.file_rel.to_str().unwrap_or("unkown file"),
			self.line as usize,
			self.col as usize,
		));
		Ok(output)
	}
}

const LINE_BUFFER_LEN: usize = 3;

fn line_number_buffer(line_no: usize) -> String {
	let line_no = line_no.to_string();
	let digits = line_no.len();
	let len = LINE_BUFFER_LEN - digits;
	" ".repeat(len)
}
