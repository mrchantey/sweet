use crate::prelude::*;
use colorize::AnsiColor;
use std::any::Any;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;
use test::TestDesc;


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LibtestHash(pub u64);

impl LibtestHash {
	pub fn new(source_file: &str, start_line: usize) -> Self {
		let mut hasher = DefaultHasher::new();
		source_file.hash(&mut hasher);
		start_line.hash(&mut hasher);
		Self(hasher.finish())
	}
}
// #[extend::ext(name=TestDescAndFnExt)]
// pub impl TestDescAndFn {
// 	fn hash(&self) -> LibtestHash { self.desc.hash() }
// }


impl std::fmt::Display for LibtestHash {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:x}", self.0)
	}
}




pub struct TestDescExt;



impl TestDescExt {
	pub fn hash(desc: &TestDesc) -> LibtestHash {
		LibtestHash::new(&desc.source_file, desc.start_line)
	}


	pub fn is_equal_location(a: &TestDesc, b: &TestDesc) -> bool {
		a.source_file == b.source_file && a.start_line == b.start_line
	}


	/// wrapper for [`test_error_location`]
	/// that works with [`TestDesc`]
	pub fn error_location(desc: &TestDesc) -> String {
		let source_file: PathBuf = desc.source_file.into();

		let stem = source_file
			.file_stem()
			.map(|stem| stem.to_string_lossy())
			.unwrap_or("unkown file".into());

		let name = Self::short_name(desc);

		test_err_location(&stem, &name)
	}


	/// The `#[test]` macro replaces results with [useless error messages](https://github.com/rust-lang/rust/blob/a25032cf444eeba7652ce5165a2be450430890ba/library/test/src/lib.rs#L234)
	/// so we instead panic and instruct user to use `unwrap`.
	/// Also used by async wasm tests, we dont care what the result is, if ya
	/// want messages, panic! at the disco
	pub fn result_to_panic<T, E>(result: Result<T, E>) {
		match result {
			Ok(_) => {}
			Err(_) => {
				panic!("test returned an Err(). Use `unwrap()` instead to see the contents of the error");
			}
		}
	}

	/// A libtest name is the fully qualified path
	/// ie `test_case::backtrace_error::test::result_builder`
	/// we want to shorten this to just the last part
	pub fn short_name(test: &TestDesc) -> String {
		let path = test.name.to_string();
		path.split("::")
			.last()
			.map(|p| p.to_string())
			.unwrap_or(path)
	}

	pub fn panic_full_format(
		desc: &TestDesc,
		panic: Box<dyn Any + Send>,
	) -> String {
		let err = panic_err_to_string(panic);
		let loc = Self::error_location(&desc);
		test_err_full_format(&loc, &err, "")
	}


	/// wasm doesnt have access to the fs so instead we just link
	/// to the `path_to_test.rs:line:col` in the console
	pub fn best_effort_full_err(desc: &TestDesc, err: &str) -> String {
		let loc = TestDescExt::error_location(&desc);

		let cwd_root = BacktraceLocation::cwd_root();
		// we dont get backtrace in wasm so just point to test start
		let backtrace = BacktraceLocation::from_test_desc(desc)
			.stack_line_string(&cwd_root);

		test_err_full_format(&loc, err, &backtrace)
	}
}

fn panic_err_to_string(e: Box<dyn Any + Send>) -> String {
	match e.downcast::<String>() {
		Ok(v) => *v,
		Err(e) => match e.downcast::<&str>() {
			Ok(v) => v.to_string(),
			_ => "Failed to convert panic to string".to_owned(),
		},
	}
}


pub fn test_err_full_format(
	location: &str,
	err: &str,
	backtrace: &str,
) -> String {
	format!("{}\n\n{}\n\n{}", location, err, backtrace)
}


/// for a given error `it failed!` format like so:
///
/// ```ignore
/// ● file_name.rs > test_name
///
///
/// it failed!
/// ```
///
pub fn test_err_location(file_stem: &str, test_name: &str) -> String {
	format!("\n● {} > {}", file_stem, test_name).red().bold()
}
