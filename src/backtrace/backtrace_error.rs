pub type BacktraceResult = Result<(), BacktraceError>;

pub trait BuildableResult<T> {
	/// Create the backtrace
	fn build_res_mapped(self) -> anyhow::Result<T>;
}
impl<T> BuildableResult<T> for Result<T, BacktraceError> {
	fn build_res_mapped(self) -> anyhow::Result<T> {
		match self {
			Err(err) => Err(err.build_inner(0)),
			Ok(val) => Ok(val),
		}
	}
}


/// A special error designed to be 'unwrapped' into anyhow
/// at the last minute, so it captures the correct frame
// TODO deprecate this
pub struct BacktraceError(pub String);

impl BacktraceError {
	pub fn new(message: impl Into<String>) -> Self { Self(message.into()) }
	// wasm cannot backtrace so instead we handle this at the test level
	#[cfg(target_arch = "wasm32")]
	pub(crate) fn build_inner(self, _: usize) -> anyhow::Error {
		return anyhow::anyhow!("{}", self.0);
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub(crate) fn build_inner(
		self,
		_additional_frames: usize,
	) -> anyhow::Error {
		// use crate::prelude::BacktraceFile;
		// use backtrace::Backtrace;
		// 3 = actual code
		// 2 = assertion: to_be(){}
		// 1 = build_err/build_res
		// 0 = build_inner
		// const FRAME_DEPTH: usize = 3;

		// without unresolved this is VERY slow >500ms
		// let bt = Backtrace::new_unresolved();

		// let backtrace_str = if let Some(frame) =
		// 	&bt.frames().get(FRAME_DEPTH - additional_frames)
		// {
		// 	let mut frame = frame.to_owned().clone();
		// 	frame.resolve();
		// 	BacktraceFile::file_context(&frame.symbols()[0]).unwrap_or(format!(
		// 		"Failed to get backtrace, file not found: {:?}",
		// 		frame.symbols()[0].filename()
		// 	))
		// } else {
		// 	"Backtrace frame not found".to_string()
		// };
		anyhow::anyhow!("{}\n\n", self.0)
	}
	pub fn build_err(self) -> anyhow::Error { self.build_inner(0) }
	pub fn build_res(self) -> anyhow::Result<()> { Err(self.build_inner(0)) }
}

impl From<&str> for BacktraceError {
	fn from(err: &str) -> Self { Self::new(err.to_string()) }
}
impl From<String> for BacktraceError {
	fn from(err: String) -> Self { BacktraceError::new(err.to_string()) }
}




#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
// wasm cant backtrace
mod test {
	use crate::prelude::*;
	// use backtrace::Backtrace;

	fn mock_to_be_error() -> anyhow::Result<()> {
		some_low_level_error_assertion().build_res()
	}

	fn some_low_level_error_assertion() -> BacktraceError {
		BacktraceError("Some error".to_string())
	}

	#[test]
	#[ignore]
	fn error_builder() -> Result<()> {
		let err = mock_to_be_error().err().unwrap().to_string();
		// empty space ensures we got the correct frame

		expect(&err).to_contain("backtrace_error.rs")?;
		expect(&err).to_contain("let err = mock_to_be")?;


		Ok(())
	}
	fn mock_to_be_result() -> anyhow::Result<()> {
		some_low_level_result_assertion().build_res_mapped()
	}

	fn some_low_level_result_assertion() -> BacktraceResult {
		Err(BacktraceError("Some error".to_string()))
	}
	#[test]
	#[ignore]
	fn result_builder() -> Result<()> {
		let err = mock_to_be_result().err().unwrap().to_string();
		// empty space ensures we got the correct frame

		expect(&err).to_contain("backtrace_error.rs").unwrap();
		expect(&err).to_contain("let err = mock_to_be")?;

		Ok(())
	}
}
