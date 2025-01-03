


/// A sweet error is the panic payload emitted by matchers:
/// ```rust
/// # use sweet::prelude::*;
/// // this will panic with a MatcherErr
/// expect(true).to_be_false();
/// ```
/// 
#[derive(Debug, Clone)]
pub struct SweetError {
	pub expected: String,
	pub received: String,
}


impl SweetError {
	pub fn new(expected: String, received: String) -> Self {
		Self { expected, received }
	}
}


impl std::fmt::Display for SweetError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"Expected: {}, Received: {}",
			self.expected, self.received
		)
	}
}
