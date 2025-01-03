



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
	#[cfg(not(target_arch = "wasm32"))]
	pub frame: backtrace::Backtrace,
}


impl SweetError {
	pub fn new(expected: String, received: String) -> Self {
		#[cfg(target_arch = "wasm32")]
		return Self { expected, received };
		#[cfg(not(target_arch = "wasm32"))]
		return Self {
			expected,
			received,
			frame:backtrace::Backtrace::new_unresolved(),
		};
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
