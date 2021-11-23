//! lib.rs --- Tenex network clients
/*!
A high-level library that provides modules for writing
API clients very quickly.

This library depends on 'rlib' modules as well as third-party
libraries for specific APIs behind feature flags.

To use this library, add one or more of the features available:

```tenex = { version = "0.1.0", features = ["google", "ipapi", "aws"] }```
*/

pub use rlib::net::Error;
pub use rlib::util::Result;

#[cfg(feature = "aws")]
pub use aws;
#[cfg(feature = "google")]
pub use google;

mod client;
#[cfg(feature = "email")]
pub mod email;
#[cfg(feature = "discord")]
pub use client::discord;
#[cfg(feature = "ipapi")]
pub use client::ipapi;
#[cfg(feature = "nws")]
pub use client::nws;
#[cfg(feature = "openai")]
pub use client::openai;
#[cfg(test)]
mod tests;
