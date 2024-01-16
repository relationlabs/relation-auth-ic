//! HTTP support for IC

pub use error::HttpError;
pub use model::*;

mod error;
mod model;

pub mod handler;

pub type HttpResult<T> = std::result::Result<T, HttpError>;
