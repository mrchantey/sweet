mod error;
#[cfg(not(target_arch = "wasm32"))]
mod impl_reqwest;
mod request;
mod response;
#[cfg(target_arch = "wasm32")]
mod impl_fetch;
pub(crate) use error::*;
use http::StatusCode;
pub use request::*;
pub use response::*;
