use crate::prelude::*;
use anyhow::Result;
use backtrace::BacktraceFrame;



impl SweetError {
	pub fn assertion_frame(&self) -> Result<&BacktraceFrame> {
		if let Some(frame) = &self.backtrace.frames().get(self.assertion_depth)
		{
			Ok(frame)
		} else {
			anyhow::bail!("Failed to find backtrace frame");
		}
	}

	pub fn backtrace_str(&self) -> Result<String> {
		let frame = self.assertion_frame()?;
		BacktraceLocation::from_unresolved_frame(&frame)?.file_context()
	}
}
