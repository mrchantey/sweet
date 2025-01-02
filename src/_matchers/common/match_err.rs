#[derive(Debug, Clone)]
pub struct MatchErr {
	pub expected: String,
	pub received: String,
}


impl MatchErr {
	pub fn new(expected: String, received: String) -> Self {
		Self { expected, received }
	}
}


impl std::fmt::Display for MatchErr {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"Expected: {}, Received: {}",
			self.expected, self.received
		)
	}
}
