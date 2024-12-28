use crate::test_runner_utils::*;
use crate::test_suite::SuiteLogger;
use forky::web::*;
use std::time::Duration;
use web_sys::console;

pub struct RunnerLoggerWasm {
	start_time: f64,
}

impl RunnerLogger for RunnerLoggerWasm {
	fn start(config: &TestRunnerConfig) -> Self {
		if !config.silent {
			console::clear();
			log_web(&Self::pretty_print_intro(&config));
		}
		let start_time = performance_now();
		Self { start_time }
	}
	fn end(self, config: &TestRunnerConfig, results: &TestRunnerResult) {
		if !config.silent {
			let duration =
				Duration::from_millis((performance_now() - self.start_time) as u64);
			let summary = results.end_str(duration);
			log_web(&summary);
		}
	}
}

pub fn log_web(val: &str) { console::log_1(&val.into()); }

#[derive(Default, Debug, Clone)]
pub struct SuiteLoggerWasm;


impl SuiteLogger for SuiteLoggerWasm {
	fn on_start(_: String) -> Self { Self }
	fn on_end(self, end_str: String) { log_web(&end_str); }
}

