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

	let name = test.desc.name.to_string();

	test_err_location(&stem, &name)
}
