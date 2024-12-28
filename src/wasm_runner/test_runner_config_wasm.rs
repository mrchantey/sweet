use crate::test_runner_utils::*;
use anyhow::Result;
use forky::web::SearchParams;
use glob::Pattern;
pub const MATCHES_KEY: &str = "m";

impl TestRunnerConfig {
	pub fn from_deno_args() -> Result<Self> {
		let window = web_sys::window().expect("no global window exists");
		let deno = js_sys::Reflect::get(&window, &"Deno".into()).unwrap();
		let args = js_sys::Reflect::get(&deno, &"args".into()).unwrap();
		let args = js_sys::Array::from(&args)
			.iter()
			.map(|arg| arg.as_string().unwrap())
			.collect::<Vec<String>>();

		Self::from_raw_args(args.into_iter())
	}



	pub fn from_search_params() -> Self {
		let silent = SearchParams::get_flag("silent");

		let matches = SearchParams::get_all(MATCHES_KEY)
			.iter()
			.map(|s| Pattern::new(&format!("*{s}*")).unwrap())
			.collect::<Vec<_>>();
		// if let Some(file) =  {
		// 	//todo error onn malformed pattern
		// 	matches.push(Pattern::new(&file).unwrap());
		// }
		Self {
			watch: false,
			parallel: false,
			silent,
			matches,
		}
	}
}
