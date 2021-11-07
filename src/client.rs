//! # API clients
/*!
The Client is used by client modules which are labelled by name. The
following modules are available:
- ipapi
- nws
- openai
- discord
*/
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "ipapi")]
pub mod ipapi;
#[cfg(feature = "nws")]
pub mod nws;
#[cfg(feature = "openai")]
pub mod openai;
