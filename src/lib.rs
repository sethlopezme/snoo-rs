//! An unofficial wrapper for the Reddit API.

#![warn(missing_docs)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;

mod snoo;
pub mod error;
mod net;
mod reddit;

pub use snoo::{Snoo, SnooBuilder};

pub mod auth {
    //! Authorization and authentication types.
    pub use reddit::auth::{AuthorizationDuration, AuthorizationUrlBuilder,
                           AuthorizationUrlBuilderError, BearerToken, ResponseType, Scope,
                           ScopeSet, SharedBearerTokenFuture};
}
