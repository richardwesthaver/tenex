//! Tenex
//!
//! A high-level library that provides modules for writing
//! API clients very quickly.
//!
//! This library depends on 'rlib' modules as well as third-party
//! libraries for specific APIs behind feature flags.
//!
//! To use this library, add one or more of the features available:
//! ```tenex = { version = "0.1.0", features = ["openai", "email", "aws"] }```
pub use rlib::net::Error;
pub use rlib::util::Result;

pub mod client;

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "aws")]
pub use crate::client::aws;
#[cfg(feature = "discord")]
pub use crate::client::discord;
#[cfg(feature = "github")]
pub use crate::client::github;
#[cfg(feature = "google")]
pub use crate::client::google;
#[cfg(feature = "ipapi")]
pub use crate::client::ipapi;
#[cfg(feature = "nws")]
pub use crate::client::nws;
#[cfg(feature = "openai")]
pub use crate::client::openai;

#[cfg(test)]
mod tests;
