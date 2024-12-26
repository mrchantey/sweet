use super::*;
use forky::prelude::*;
use std::path::Path;
use sweet::native::*;
use sweet::prelude::*;

pub fn suite(cases: Vec<TestCaseNative>) -> TestSuiteNative {
	TestSuiteNative {
		file: Path::new(file!()).to_forward_slash(),
		tests: cases,
		config: Default::default(),
	}
}

#[sweet_test]
fn works() -> Result<()> {
	let _suite = suite(vec![case(TestCaseNativeFunc::Parallel(|| {
		Box::pin(async {
			panic!("hello");
		})
	}))]);

	Ok(())
}
