use crate::prelude::*;
use anyhow::Result;
use colorize::*;


/// Log cases in the default test style 🤮
#[derive(Debug, Default)]
pub struct VanillaCaseLogger;

impl CaseLogger for VanillaCaseLogger {
	fn on_result(&mut self, result: &TestDescAndResult) -> Result<()> {
		let status = match &result.result {
			TestResult::Pass => "ok".green(),
			TestResult::Fail(msg) => {
				let status = "FAILED".red();
				format!("{}\n{}", status, msg)
			}
			TestResult::Ignore(None) => "ignored".yellow(),
			TestResult::Ignore(Some(msg)) => format!("ignored, {msg}").yellow(),
		};
		crate::log!("test {} ... {}", result.desc.name.to_string(), status,);

		Ok(())
	}
}
