//! An XML-RPC implementation in Rust.
//!
//! The `xmlrpc` crate provides a minimal implementation of the [XML-RPC specification].
//!
//! [XML-RPC specification]: http://xmlrpc.scripting.com/spec.html

#![doc(html_root_url = "https://docs.rs/xmlrpc/0.14.0")]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

#[cfg(feature = "async")]
mod async_transport;
mod error;
mod parser;
mod request;
#[cfg(not(feature = "async"))]
mod transport;
mod utils;
mod value;

#[cfg(feature = "async")]
use self::async_transport as transport;

pub use error::{Error, Fault};
pub use request::Request;
pub use transport::Transport;
pub use value::{Index, Value};

#[cfg(feature = "http")]
pub use transport::http;
