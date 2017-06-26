//TODO #![deny(warnings, missing_docs)]

//! Search REST APIs and related utility functions.
//!

extern crate bytes;
extern crate futures;
extern crate httparse;
extern crate base64;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;
extern crate tokio_proto;
extern crate tokio_service;
extern crate io_dump;


pub mod ebay;
pub mod http;
pub mod request;
pub mod response;


pub use ebay::base64_credentials;
pub use ebay::oauth_body;
pub use ebay::oauth_header;
pub use ebay::oauth_token;