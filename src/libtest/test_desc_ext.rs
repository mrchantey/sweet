use crate::prelude::test_err_location;
use crate::prelude::TestRunnerConfig;
use crate::prelude::*;
use std::any::Any;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;
use test::ShouldPanic;
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
	fn hash(desc: &TestDesc) -> LibtestHash {
		LibtestHash::new(&desc.source_file, desc.start_line)
	}

	/// A failing test that should_panic is actually a success, etc
	pub fn parse_result(
		desc: &TestDesc,
		result: Result<(), String>,
	) -> Result<(), String> {
		match (result, desc.should_panic) {
			(Ok(_), ShouldPanic::No) => Ok(()),
			(Ok(_), ShouldPanic::Yes) => Err(format!("Expected panic")),
			(Ok(_), ShouldPanic::YesWithMessage(msg)) => {
				Err(format!("Expected panic: {}", msg))
			}
			(Err(err), ShouldPanic::No) => Err(err),
			(Err(_), ShouldPanic::Yes) => Ok(()),
			(Err(_), ShouldPanic::YesWithMessage(_)) => Ok(()),
		}
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

	pub fn format_panic(desc: &TestDesc, panic: Box<dyn Any + Send>) -> String {
		let err = panic_err_to_string(panic);
		let loc = Self::error_location(&desc);
		test_err_full_format(&loc, &err, "")
	}

	/// Checks both the file path and the full test name
	///
	/// for matcher `foo` the following will pass:
	/// - path: `/src/foo/bar.rs`
	/// - name: `crate::foo::test::it_works`
	pub fn passes_filter(desc: &TestDesc, config: &TestRunnerConfig) -> bool {
		let path = desc.source_file;
		let name = desc.name.to_string();
		config.matches.len() == 0
			|| config.matches.iter().any(|a| a.matches(&path))
			|| config.matches.iter().any(|a| a.matches(&name))
	}
}
