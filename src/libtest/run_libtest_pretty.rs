use crate::prelude::*;



#[deprecated = "use custom runner"]
pub fn run_libtest_pretty(tests: &[&test::TestDescAndFn]) {
	return test_main_with_filenames(tests);
}

/// Pretty much run libtest as-is but with pretty filenames for unit tests.
fn test_main_with_filenames(tests: &[&test::TestDescAndFn]) {
	let tests = apply_filenames(tests);
	let tests = tests.iter().collect::<Vec<_>>();
	println!("\n🤘 sweet as! 🤘\n");
	test::test_main_static(&tests);
}

fn apply_filenames(tests: &[&test::TestDescAndFn]) -> Vec<test::TestDescAndFn> {
	tests
		.into_iter()
		.map(|test| {
			let mut test = clone_libtest(test);
			test.desc.name = test::DynTestName(format!(
				"{} - {}",
				test.desc.source_file,
				libtest_short_name(&test)
			));
			// test::StaticTestName(test.desc.source_file);
			test
		})
		.collect()
}
