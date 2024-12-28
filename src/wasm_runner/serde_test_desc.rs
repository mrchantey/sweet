use serde::Deserialize;
use serde::Serialize;
use test::ShouldPanic;
use test::TestDesc;
use test::TestType;


/// Serializable and deserializable version of `TestDesc`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerdeTestDesc {
	// test: TestDescAndFn,
	name: String,
	pub ignore: bool,
	ignore_message: Option<String>,
	source_file: String,
	pub start_line: usize,
	pub start_col: usize,
	pub end_line: usize,
	pub end_col: usize,
	should_panic: bool,
	should_panic_message: Option<String>,
	pub compile_fail: bool,
	pub no_run: bool,
	test_type: SerdeTestType,
}

impl SerdeTestDesc {
	pub fn new(desc: &TestDesc) -> Self { desc.clone().into() }
}

impl Into<SerdeTestDesc> for TestDesc {
	fn into(self) -> SerdeTestDesc {
		SerdeTestDesc {
			name: self.name.to_string(),
			ignore: self.ignore,
			ignore_message: self.ignore_message.map(|s| s.to_string()),
			source_file: self.source_file.to_string(),
			start_line: self.start_line,
			start_col: self.start_col,
			end_line: self.end_line,
			end_col: self.end_col,
			should_panic: match self.should_panic {
				ShouldPanic::Yes | ShouldPanic::YesWithMessage(_) => true,
				ShouldPanic::No => false,
			},
			should_panic_message: match self.should_panic {
				ShouldPanic::YesWithMessage(msg) => Some(msg.to_string()),
				_ => None,
			},
			compile_fail: self.compile_fail,
			no_run: self.no_run,
			test_type: self.test_type.into(),
		}
	}
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SerdeTestType {
	/// Unit-tests are expected to be in the `src` folder of the crate.
	UnitTest,
	/// Integration-style tests are expected to be in the `tests` folder of the crate.
	IntegrationTest,
	/// Doctests are created by the `librustdoc` manually, so it's a different type of test.
	DocTest,
	/// Tests for the sources that don't follow the project layout convention
	/// (e.g. tests in raw `main.rs` compiled by calling `rustc --test` directly).
	Unknown,
}


impl Into<TestType> for SerdeTestType {
	fn into(self) -> TestType {
		match self {
			SerdeTestType::UnitTest => TestType::UnitTest,
			SerdeTestType::IntegrationTest => TestType::IntegrationTest,
			SerdeTestType::DocTest => TestType::DocTest,
			SerdeTestType::Unknown => TestType::Unknown,
		}
	}
}

impl Into<SerdeTestType> for TestType {
	fn into(self) -> SerdeTestType {
		match self {
			TestType::UnitTest => SerdeTestType::UnitTest,
			TestType::IntegrationTest => SerdeTestType::IntegrationTest,
			TestType::DocTest => SerdeTestType::DocTest,
			TestType::Unknown => SerdeTestType::Unknown,
		}
	}
}
