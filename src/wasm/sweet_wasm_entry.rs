use super::*;
// use crate::TestRunnerConfig;
use anyhow::Result;
use forky_web::*;
use leptos::*;


pub fn interactive_mode() -> bool { false == SearchParams::get_flag("run") }

pub fn sweet_wasm_entry() -> Result<()> {
	match entry() {
		Ok(_) => Ok(()),
		Err(e) => {
			let err = format!("Sweet Internal Error:\n\n{}", e.to_string());
			web_sys::console::error_1(&err.into());
			Err(e)
		}
	}
}

fn entry() -> Result<()> {
	forky_web::set_panic_hook();
	if let Some(testid) = SearchParams::get("testid") {
		TestRunnerWasm::run_case(testid.parse().unwrap())
	} else if interactive_mode() {
		mount_to_body(|| {
			view! {
				<link rel="stylesheet" href="sweet-style.css"/>
				<Root/>
			}
		});
		Ok(())
	} else {
		mount_to_body(|| view! { <Root/> });
		Ok(())
	}
}
