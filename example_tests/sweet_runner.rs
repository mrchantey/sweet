//! ** If you run nightly you probably want the `libtest_runner` example**
//!
//! The sweet runner is a way to enable sweet in stable rust.
//! It cannot detect `#[test]` or `#[tokio::test]` macros so
//! ony `#[sweet::test]` may be used
//!


// This will pull in a `main` function for tests.
// #![no_main]
#[cfg(test)]
pub use sweet::main;

#[cfg(not(test))]
fn main() {}

#[cfg(test)]
mod test {
	use sweet::prelude::*;

	#[sweet::test]
	fn works() -> Result<()> {
		expect(false).to_be_false()?;

		Ok(())
	}
}
