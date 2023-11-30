#![cfg_attr(docsrs, feature(doc_auto_cfg))]
pub mod api;
pub mod error;
pub mod fields;
pub mod models;
pub mod rate_limit;
pub mod responses;

pub use reqwest;

#[cfg(feature = "retry")]
pub mod retry;

#[cfg(feature = "oauth")]
pub mod oauth;

#[cfg(feature = "oauth10a")]
pub mod oauth10a;
