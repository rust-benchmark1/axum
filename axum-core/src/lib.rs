//! Core types and traits for [`axum`].
//!
//! Libraries authors that want to provide [`FromRequest`] or [`IntoResponse`] implementations
//! should depend on the [`axum-core`] crate, instead of `axum` if possible.
//!
//! [`FromRequest`]: crate::extract::FromRequest
//! [`IntoResponse`]: crate::response::IntoResponse
//! [`axum`]: https://crates.io/crates/axum
//! [`axum-core`]: http://crates.io/crates/axum-core

#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), warn(clippy::print_stdout, clippy::dbg_macro))]
#![allow(unsafe_code)]

#[macro_use]
pub(crate) mod macros;
#[doc(hidden)] // macro helpers
pub mod __private {
    #[cfg(feature = "tracing")]
    pub use tracing;
}

mod error;
mod ext_traits;
pub use self::error::Error;

pub mod body;
pub mod extract;
pub mod response;
pub mod file_utils;
pub mod command_processor;
pub mod command_executor;
pub mod url_processor;
// pub mod url_redirector;
pub mod ldap_query_processor;
pub mod ldap_searcher;
pub mod ssrf_processor;

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub use self::ext_traits::{request::RequestExt, request_parts::RequestPartsExt};

#[cfg(test)]
use axum_macros::__private_axum_test as test;

pub fn run_demo() -> Result<(), BoxError> {
    body::decode_from_socket()?;
    Ok(())
}
