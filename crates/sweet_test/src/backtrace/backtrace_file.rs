use crate::libtest::test_err_link;
use ::test::TestDesc;
use anyhow::Result;
use backtrace::BacktraceFrame;
use colorize::*;
use forky::prelude::*;
use std::panic::PanicHookInfo;
use std::path::Path;
use std::path::PathBuf;

pub struct BacktraceFile;

impl BacktraceFile {
	pub fn file_context_from_desc(desc: &TestDesc) -> Result<String> {
		Self::file_context(
			Path::new(&desc.source_file),
			desc.start_line,
			desc.start_col,
		)
	}

	/// will fall back to desc if no location is found
	pub fn file_context_from_panic(
		info: &PanicHookInfo,
		desc: &TestDesc,
	) -> Result<String> {
		if let Some(location) = info.location() {
			Self::file_context(
				Path::new(&location.file()),
				location.line() as usize,
				location.column() as usize,
			)
		} else {
			Self::file_context_from_desc(desc)
		}
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


	pub const LINE_CONTEXT_SIZE: usize = 2;


	/// # Errors
	/// This function will return an error if the file cannot be read
	pub fn file_context(
		path: &Path,
		line_no: usize,
		col_no: usize,
	) -> Result<String> {
		let path = with_sweet_root(path);
		let file = read_file(&path)?;
		let lines: Vec<&str> = file.split("\n").collect();
		//line number is one-indexed
		let start =
			usize::max(0, line_no.saturating_sub(Self::LINE_CONTEXT_SIZE + 1));
		let end =
			usize::min(lines.len() - 1, line_no + Self::LINE_CONTEXT_SIZE);

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
			&PathExt::relative(&path).unwrap_or(&path).to_string_lossy(),
			line_no,
			col_no,
		));
		Ok(output)
	}
}
/// Prefix the path with $SWEET_ROOT if it exists
fn with_sweet_root(path: &Path) -> PathBuf {
	if let Some(sweet_root) = std::env::var("SWEET_ROOT").ok() {
		let mut root = PathBuf::from(sweet_root);
		root.push(path);
		root
	} else {
		path.to_path_buf()
	}
}

fn read_file(path: &Path) -> Result<String> {
	let bail = |cwd: &str| {
		anyhow::anyhow!(
			"Failed to read file:\ncwd:\t{}\npath:\t{}\n{CONTEXT}",
			cwd,
			&path.display()
		)
	};

	const CONTEXT: &str = r#"
This can happen when working with workspaces and the sweet root has not been set.
Please ensure the following is configured in `.cargo/config.toml`:

[env]
SWEET_ROOT = { value = "", relative = true }

"#;

	#[cfg(target_arch = "wasm32")]
	let file = wasm_fs::read_file(&path.to_string_lossy().to_string())
		.ok_or_else(|| bail(&wasm_fs::cwd()))?;
	#[cfg(not(target_arch = "wasm32"))]
	let file = ReadFile::to_string(path).map_err(|_| {
		bail(
			&std::env::current_dir()
				.unwrap_or_default()
				.display()
				.to_string(),
		)
	})?;

	Ok(file)
}


const LINE_BUFFER_LEN: usize = 3;

fn line_number_buffer(line_no: usize) -> String {
	let line_no = line_no.to_string();
	let digits = line_no.len();
	let len = LINE_BUFFER_LEN.saturating_sub(digits);
	" ".repeat(len)
}


#[cfg(target_arch = "wasm32")]
pub mod wasm_fs {
	use wasm_bindgen::prelude::*;
	#[wasm_bindgen]
	extern "C" {
		pub fn cwd() -> String;
	}
	#[wasm_bindgen]
	extern "C" {
		pub fn read_file(path: &str) -> Option<String>;
	}
}


#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod test {
	use crate::prelude::*;

	#[test]
	fn works() {
		expect(wasm_fs::read_file("foobar")).to_be_none();
		expect(wasm_fs::read_file("Cargo.toml")).to_be_some();
	}
}
