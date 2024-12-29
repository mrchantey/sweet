use crate::test_case::*;
use anyhow::Error;
use futures::Future;


pub async fn run_cases_series(to_run: Vec<&impl TestCase>) -> Vec<Error> {
	let mut results = Vec::with_capacity(to_run.len());
	for case in to_run {
		if let Err(result) = case.run().await {
			results.push(result);
		}
	}
	results
}

pub async fn run_cases_series_with_before<Case, Fut>(
	to_run: Vec<&Case>,
	mut before: impl FnMut(&Case) -> Fut,
) -> Vec<Error>
where
	Case: TestCase,
	Fut: Future<Output = ()>,
{
	let mut results = Vec::with_capacity(to_run.len());
	for case in to_run {
		before(case).await;
		if let Err(result) = case.run().await {
			results.push(result);
		}
	}
	results
}

// #[derive(Default, Debug, Clone)]
// pub struct TestSuite<Case>
// where
// 	Case: TestCase,
// {
// 	pub file: String,
// 	pub tests: Vec<Case>,
// 	pub config: TestSuiteConfig,
// }

// impl<Case> TestSuiteTrait<Case> for TestSuite<Case>
// where
// 	Case: TestCase,
// {
// 	fn new(file: String) -> Self {
// 		Self {
// 			file,
// 			tests: Vec::new(),
// 			config: Default::default(),
// 		}
// 	}
// 	fn file(&self) -> &str { self.file.as_str() }
// 	fn config(&self) -> &TestSuiteConfig { &self.config }
// 	fn tests(&self) -> &Vec<Case> { &self.tests }
// 	fn tests_mut(&mut self) -> &mut Vec<Case> { &mut self.tests }
// }
