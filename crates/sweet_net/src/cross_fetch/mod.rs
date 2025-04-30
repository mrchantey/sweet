#[cfg(not(target_arch = "wasm32"))]
mod native;
mod error;
mod request;
mod response;
#[cfg(target_arch = "wasm32")]
mod wasm;
pub use error::*;
use http::StatusCode;
pub use request::*;
pub use response::*;

