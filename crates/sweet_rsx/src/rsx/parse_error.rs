use forky::prelude::FsError;
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
	#[error("{0}")]
	Fs(FsError),
	#[error("Hydration Error: {0}")]
	Hydration(String),
	#[error("Parse Error: {0}")]
	Other(String),
}

impl From<FsError> for ParseError {
	fn from(e: FsError) -> Self { Self::Fs(e) }
}

impl From<anyhow::Error> for ParseError {
	fn from(e: anyhow::Error) -> Self { Self::Other(e.to_string()) }
}
impl From<String> for ParseError {
	fn from(e: String) -> Self { Self::Other(e) }
}
impl From<&str> for ParseError {
	fn from(e: &str) -> Self { Self::Other(e.to_string()) }
}
