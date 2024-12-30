use test::TestDescAndFn;
use test::TestFn;




pub struct TestDescAndFnExt;


impl TestDescAndFnExt {
	/// copied from https://github.com/rust-lang/rust/blob/a25032cf444eeba7652ce5165a2be450430890ba/library/test/src/lib.rs#L223
	pub fn clone(test: &TestDescAndFn) -> TestDescAndFn {
		match test.testfn {
			TestFn::StaticTestFn(f) => TestDescAndFn {
				testfn: TestFn::StaticTestFn(f),
				desc: test.desc.clone(),
			},
			// TestFn::StaticBenchFn(f) => TestDescAndFn {
			// 	testfn: TestFn::StaticBenchFn(f),
			// 	desc: test.desc.clone(),
			// },
			_ => panic!("non-static tests are not supported"),
		}
	}

	pub fn func(test: &TestDescAndFn) -> fn() -> Result<(), String> {
		match test.testfn {
			TestFn::StaticTestFn(func) => func,
			_ => panic!("non-static tests are not supported"),
		}
	}
}
