use forky::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use sweet::prelude::*;
use sweet::test_case::*;

struct Case;
impl TestCase for Case {
	fn name(&self) -> &'static str { "works" }
	fn config(&self) -> &TestCaseConfig { todo!() }
	fn path(&self) -> PathBuf { Path::new(file!()).to_forward_slash() }
	async fn run_func(&self) -> Result<()> { Ok(()) }
}

#[test]
fn works() -> Result<()> {
	let case = Case;
	expect(case.path().to_str().unwrap()).to_be("test/common/test_case.rs")?;
	Ok(())
}
