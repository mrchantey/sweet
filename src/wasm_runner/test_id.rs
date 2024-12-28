use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
static CNT: AtomicUsize = AtomicUsize::new(0);

/// Create unique test ids
#[derive(Debug, Clone, Copy)]
pub struct TestId(usize);

impl Deref for TestId {
	type Target = usize;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::fmt::Display for TestId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl TestId {
	pub fn inner(&self) -> usize { self.0 }

	pub fn next() -> Self { Self(CNT.fetch_add(1, Ordering::SeqCst)) }

	pub fn var_name(&self) -> String { format!("test_output_{}", self.0) }
}
