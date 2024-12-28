use crate::prelude::*;



pub fn run_suite(suite: &LibtestSuite) -> SuiteResult {
	let mut skipped = 0;
	let mut num_ran = 0;
	let failures = suite
		.tests
		.iter()
		.filter_map(|test| {
			if test.desc.ignore {
				skipped += 1;
				return None;
			}
			num_ran += 1;
			match run_test(test) {
				Ok(_) => None,
				Err(err) => Some(err),
			}
		})
		.collect::<Vec<_>>();

	let result = SuiteResult::new(suite.source_file.into(), num_ran, skipped)
		.with_failed(failures);
	log_web(&result.end_str());
	result
}
