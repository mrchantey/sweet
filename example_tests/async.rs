#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
use std::cell::RefCell;
use std::rc::Rc;
use sweet::prelude::*;
// use sweet::prelude::*;
#[test]
// #[should_panic]
fn async_test() {
	SweetTestCollector::register(Box::pin(async {
		// this is better than how #[test] does it anyway
		// #[test] discards the error and returns a useless one
		if let Err(err) = my_resulty_test().await {
			panic!("{:?}", err);
		}
	}));
	// SweetTestCollector::register(Box::pin(my_test()));
	// sweet::prelude::panic_with_id(id);
}

// async fn my_resulty_test() -> Result<(), String> { Ok(()) }
async fn my_resulty_test() -> Result<(), String> { Err("foo".to_string()) }

async fn my_test() {
	// wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(
	// 	&mut |resolve, _| {
	// 		web_sys::window()
	// 			.unwrap()
	// 			.set_timeout_with_callback_and_timeout_and_arguments_0(
	// 				&resolve, 100,
	// 			)
	// 			.unwrap();
	// 	},
	// ))
	// .await
	// .unwrap();
	// panic!("pizza football {}", val);
}


// AsyncTestDescWithFn::register(AsyncTestDescWithFn {
// 	desc: sweet::exports::TestDesc {
// 		name: todo!(),
// 		ignore: todo!(),
// 		ignore_message: todo!(),
// 		source_file: todo!(),
// 		start_line: todo!(),
// 		start_col: todo!(),
// 		end_line: todo!(),
// 		end_col: todo!(),
// 		should_panic: todo!(),
// 		compile_fail: todo!(),
// 		no_run: todo!(),
// 		test_type: todo!(),
// 	},
// 	func: Box::pin(my_test("foobar")),
// 	result: Rc::new(RefCell::new(Ok(()))),
// });
