use crate::prelude::*;
use std::path::PathBuf;
use test::TestDescAndFn;

/// wrapper for [`test_error_location`]
/// that works with [`TestDescAndFn`]
pub fn libtesttest_error_location(test: &TestDescAndFn) -> String {
	let source_file: PathBuf = test.desc.source_file.into();

	let stem = source_file
		.file_stem()
		.map(|stem| stem.to_string_lossy())
		.unwrap_or("unkown file".into());

	let name = libtest_short_name(test);

	test_err_location(&stem, &name)
}


/// The `#[test]` macro replaces results with [useless error messages](https://github.com/rust-lang/rust/blob/a25032cf444eeba7652ce5165a2be450430890ba/library/test/src/lib.rs#L234)
/// so we instead panic and instruct user to use `unwrap`
pub fn libtest_result_to_panic(result: Result<(), String>) {
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
pub fn libtest_short_name(test: &TestDescAndFn) -> String {
	let path = test.desc.name.to_string();
	path.split("::")
		.last()
		.map(|p| p.to_string())
		.unwrap_or(path)
}


/// Checks both the file path and the full test name
///
/// for matcher `foo` the following will pass:
/// - path: `/src/foo/bar.rs`
/// - name: `crate::foo::test::it_works`
pub fn libtest_passes_filter(
	config: &TestRunnerConfig,
	test: &TestDescAndFn,
) -> bool {
	let path = test.desc.source_file;
	let name = test.desc.name.to_string();
	config.matches.len() == 0
		|| config.matches.iter().any(|a| a.matches(&path))
		|| config.matches.iter().any(|a| a.matches(&name))
}
