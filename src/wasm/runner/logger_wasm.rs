use crate::test_runner::*;
use crate::test_suite::SuiteLogger;
use forky::web::*;
use std::time::Duration;
use wasm_bindgen::JsValue;
use web_sys::console;

pub struct RunnerLoggerWasm {
	start_time: f64,
}


impl RunnerLogger for RunnerLoggerWasm {
	fn start(config: &TestRunnerConfig) -> Self {
		console::clear();
		let intro = Self::pretty_print_intro(&config);
		log_web(&intro);
		let start_time = performance_now();
		Self { start_time }
	}
	fn end(self, results: &TestRunnerResult) {
		let duration =
			Duration::from_millis((performance_now() - self.start_time) as u64);
		let summary = results.end_str(duration);
		log_web(&summary);
	}
}

pub fn log_web(val: &str) { console::log_1(&val.into()); }

#[derive(Default, Debug, Clone)]
pub struct SuiteLoggerWasm;


impl SuiteLogger for SuiteLoggerWasm {
	fn on_start(_: String) -> Self { Self }
	fn on_end(self, end_str: String) { log_web(&end_str); }
}

/// Prepare the panic hook to collect the panic message
pub fn set_panic_hook(id: usize) {
	std::panic::set_hook(Box::new(move |panic_info| {
		let payload = panic_info.payload_as_str().unwrap_or("no panic message");
		set_test_output(id, payload);
	}));
}

fn set_test_output(id: usize, value: &str) {
	let window = web_sys::window().expect("no global window exists");
	js_sys::Reflect::set(
		&window,
		&JsValue::from_str(format!("test_output_{}", id).as_str()),
		&JsValue::from_str(value),
	)
	.unwrap();
}

/// Collect the message from the panic hook
pub fn get_test_output(id: usize) -> String {
	let window = web_sys::window().expect("no global window exists");
	let output = js_sys::Reflect::get(
		&window,
		&JsValue::from_str(format!("test_output_{}", id).as_str()),
	)
	.unwrap();
	output.as_string().unwrap()
}
