use crate::test_runner_utils::*;
use std::time::Instant;

pub struct RunnerLoggerNative {
	start_time: Instant,
}
impl RunnerLogger for RunnerLoggerNative {
	fn start(config: &TestRunnerConfig) -> Self {
		if !config.silent {
			pretty_env_logger::try_init().ok();
			let intro = Self::pretty_print_intro(&config);
			println!("{intro}");
		}

		let start_time = Instant::now();
		Self { start_time }
	}
	fn end(self, config: &TestRunnerConfig, results: &TestRunnerResult) {
		if !config.silent {
			let duration = self.start_time.elapsed();
			let summary = results.end_str(duration);
			println!("{summary}");
		}
	}
}
