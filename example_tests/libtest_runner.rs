//! An example of how a top level file (main.rs, lib.rs, etc) can collect the existing `#[test]` macro.
//! This requires the `collect_libtest` feature which is enabled by default.
#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]


fn main() {}


#[cfg(test)]
mod test {
	use sweet::prelude::*;

	#[test]
	fn works() -> Result<()> {
		// expect(true).to_be_true().unwrap();

		anyhow::bail!("ita faila");
		// Ok(())
	}
}
