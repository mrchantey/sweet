use crate::libtest::test_err_link;
use anyhow::Result;
use backtrace::Backtrace;
use backtrace::BacktraceFrame;
use colorize::*;
use forky::prelude::*;
use std::fs;
use std::path::Path;
use test::TestDesc;

pub struct BacktraceFile;

impl BacktraceFile {
	pub fn backtrace_str(depth: usize) -> Result<String> {
		let bt = Backtrace::new_unresolved();
		let frame = Self::get_frame(&bt, depth)?;
		Self::file_context_from_frame(&frame)
	}

	pub fn get_frame(bt: &Backtrace, depth: usize) -> Result<BacktraceFrame> {
		if let Some(frame) = &bt.frames().get(depth) {
			let mut frame = frame.to_owned().clone();
			frame.resolve();
			Ok(frame)
		} else {
			anyhow::bail!("Failed to find backtrace frame");
		}
	}

	pub fn file_context_from_desc(desc: &TestDesc) -> Result<String> {
		let file = Path::new(&desc.source_file);
		Self::file_context(file, desc.start_line, desc.start_col)
	}


	pub fn file_context_from_frame(frame: &BacktraceFrame) -> Result<String> {
		let symbol = frame
			.symbols()
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No symbols"))?;
		let file = symbol
			.filename()
			.ok_or_else(|| anyhow::anyhow!("Bactrace has no file"))?;

		let line_no = symbol.lineno().unwrap_or_default() as usize;
		let col_no = symbol.colno().unwrap_or_default() as usize;

		Self::file_context(file, line_no, col_no)
	}


	/// # Errors
	/// This function will return an error if the file cannot be read
	pub fn file_context(
		file: &Path,
		line_no: usize,
		col_no: usize,
	) -> Result<String> {
		//line number is one-indexed
		const LINE_CONTEXT_SIZE: usize = 2;
		let lines = fs::read_to_string(file)?;
		let lines: Vec<&str> = lines.split("\n").collect();
		let start =
			usize::max(0, line_no.saturating_sub(LINE_CONTEXT_SIZE + 1));
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
			&file.relative().unwrap_or(file).to_string_lossy(),
			line_no,
			col_no,
		));
		Ok(output)
	}
}

const LINE_BUFFER_LEN: usize = 3;

fn line_number_buffer(line_no: usize) -> String {
	let line_no = line_no.to_string();
	let digits = line_no.len();
	let len = LINE_BUFFER_LEN.saturating_sub(digits);
	" ".repeat(len)
}
