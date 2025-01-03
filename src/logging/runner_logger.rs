use crate::prelude::*;
use colorize::*;
use std::sync::Arc;
use std::time::Duration;
use test::TestDescAndFn;

pub struct RunnerLogger {
	#[cfg(not(target_arch = "wasm32"))]
	start_time: std::time::Instant,
	#[cfg(target_arch = "wasm32")]
	start_time: f64,
	case_logger: CaseLoggerEnum,
	config: Arc<TestRunnerConfig>,
	cases: Vec<TestDescAndResult>,
}

impl RunnerLogger {
	pub fn start(
		config: Arc<TestRunnerConfig>,
		tests: &[&TestDescAndFn],
	) -> Self {
		let case_logger = CaseLoggerEnum::new(config.clone(), tests);

		#[cfg(not(target_arch = "wasm32"))]
		{
			use forky::prelude::terminal;
			if !config.quiet && config.watch {
				terminal::clear();
			}
			if !config.quiet {
				crate::log!("\n🤘 sweet as! 🤘\n\n{config}")
			}

			Self {
				start_time: std::time::Instant::now(),
				cases: Vec::new(),
				case_logger,
				config,
			}
		}
		#[cfg(target_arch = "wasm32")]
		{
			use forky::web::*;
			use web_sys::console;
			if !config.quiet && config.watch {
				console::clear();
			}
			if !config.quiet {
				crate::log!("\n🤘 sweet as! 🤘\n\n{config}")
			}
			let start_time = performance_now();
			Self {
				start_time,
				case_logger,
				cases: Vec::new(),
				config,
			}
		}
	}
	pub fn on_result(&mut self, result: TestDescAndResult) -> Result<()> {
		if !self.config.quiet {
			self.case_logger.on_result(&result)?;
		}
		self.cases.push(result);
		Ok(())
	}

	/// Total duration of all tests run
	fn duration(&self) -> Duration {
		#[cfg(not(target_arch = "wasm32"))]
		return self.start_time.elapsed();
		#[cfg(target_arch = "wasm32")]
		return Duration::from_millis(
			(forky::web::performance_now() - self.start_time) as u64,
		);
	}

	/// Finalize outputs and exit with code 1 if failed
	pub fn end(mut self) {
		let result_count = ResultCount::from_case_results(&self.cases);

		if !self.config.quiet {
			crate::log_val(&self.case_results(&result_count));
		}
		self.on_results_printed();
		if !self.config.watch && !result_count.succeeded() {
			// std::process::exit(1);
		}
	}

	fn on_results_printed(&mut self) {}
	fn case_results(&mut self, results: &ResultCount) -> String {
		let mut post_run = String::from("\n");

		if results.is_empty() {
			post_run += "No Tests Found\n".red().as_str();
			return post_run;
		} else if results.succeeded() {
			post_run +=
				"All tests passed\n".bold().cyan().underlined().as_str();
		}

		if let Some(case_logger_end_str) = self.case_logger.end_str() {
			post_run += case_logger_end_str.as_str();
			post_run.push('\n');
		}

		// post_run += suites.pretty_print("Suites:\t\t").as_str();
		// post_run.push('\n');
		post_run += results.pretty_print("Tests").as_str();
		post_run.push('\n');
		post_run += print_time(self.duration()).as_str();
		post_run
	}
}

fn print_time(duration: Duration) -> String {
	let millis = duration.as_millis();
	let prefix = "Time: \t\t".bold();
	if millis < 100 {
		format!("{}{} ms\n\n", prefix, millis)
	} else {
		format!("{}{:.2} s\n\n", prefix, millis as f32 * 0.001)
	}
}
