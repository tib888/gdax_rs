//! This crate is a wrapper around GDAX API ([https://docs.gdax.com](https://docs.gdax.com/)).
//!
//! At this stage it support **only the public end points**.
//!
//! # How it works
//!
//! There are two main concept to understand how it works :
//!
//! - The [`RESTClient`](struct.RESTClient.html) is in charge of handling all HTTPS connection with the API.
//! - The structs implementing `EndPointRequest` trait that are able to create a request for a defined end point,
//!   all the public end points of the gdax documentation have there request and there response implemented.
//!
//! The public modules are organised following the same tree as the [GDAX documentation](https://docs.gdax.com/).
//!
//! # Examples

extern crate chrono;
extern crate futures;
// #[macro_use]
extern crate hyper;
extern crate hyper_tls;
extern crate itertools;
extern crate mockito;
extern crate percent_encoding;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;

pub mod products;
pub mod currencies;
pub mod time;
mod url;
mod rest_client;
mod error;
mod serde_util;
pub use rest_client::{Cursor, Pagination, RESTClient};
