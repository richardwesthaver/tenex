//! # API clients
//!
//! The Client is used by client modules which are labelled by name. The
//! following modules are available:
//! - ipapi
//! - nws
//! - openai
use std::env;

#[cfg(feature = "aws")]
pub mod aws;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "google")]
pub mod google;
#[cfg(feature = "ipapi")]
pub mod ipapi;
#[cfg(feature = "nws")]
pub mod nws;
#[cfg(feature = "openai")]
pub mod openai;

/// User-Agent HTTP Header value
pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
