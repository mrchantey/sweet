use sweet::*;


// how wasm-bindgen-test unwinds panics
//https://github.com/rustwasm/wasm-bindgen/blob/74bfc1f85ead6a3e0c37a86e5f93df3e692e217a/crates/test/src/rt/mod.rs#L227-L240

sweet! {

	//TODO should panic
	it skip "handles panics"{
		assert!(true ==false);
	}
	it skip "fails"{
		expect(true).to_be_false()?;
	}
}
