#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod test {
	// use forky::prelude::FsExt;
	// use forky::prelude::ReadFile;
	// use sweet::prelude::*;

	#[test]
	fn works() {



		// like parse_file but output html too
	}
}
