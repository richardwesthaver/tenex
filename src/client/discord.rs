//! Discord API client
use serde::{Deserialize, Serialize};
/// The user data we'll get back from Discord.
/// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub avatar: Option<String>,
  pub username: String,
  pub discriminator: String,
}
