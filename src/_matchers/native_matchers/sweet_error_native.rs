use crate::prelude::*;
use anyhow::Result;
use backtrace::BacktraceFrame;



impl SweetError {
	pub fn assertion_frame(&self) -> Result<BacktraceFrame> {
		if let Some(frame) = &self.backtrace.frames().get(self.assertion_depth)
		{
			let mut frame = frame.to_owned().clone();
			frame.resolve();
			Ok(frame)
		} else {
			anyhow::bail!("Failed to find backtrace frame");
		}
	}

	pub fn backtrace_str(&self) -> Result<String> {
		let frame = self.assertion_frame()?;
		BacktraceFile::file_context_from_frame(&frame)
	}
}
