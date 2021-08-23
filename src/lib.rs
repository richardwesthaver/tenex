//! Tenex
//!
//! A high-level library that provides modules for writing
//! API clients very quickly.
//!
//! This library depends on 'rlib' modules, which contains all
//! low-level implementation details, as well as third-party libraries
//! for specific APIs behind feature flags.
//!
//! To use this library, add one or more of the features available:
//! ```tenex = { version = "0.1.0", features = ["openai", "email", "aws"] }```
#[cfg(feature = "net")]
pub mod client;
#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "ipapi")]
pub use crate::client::ipapi;
#[cfg(feature = "nws")]
pub use crate::client::nws;
#[cfg(feature = "openai")]
pub use crate::client::openai;

#[cfg(test)]
mod tests;
