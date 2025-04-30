use http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;



#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Request error: {0}")]
	ResponseNotOk(StatusCode),
	#[error("Network error: {0}")]
	NetworkError(String),
	#[error("Serialization failed: {0}")]
	Serialization(String),
	#[error("Unknown error")]
	Unknown,
}
