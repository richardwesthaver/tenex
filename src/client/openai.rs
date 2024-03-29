use crate::Result;
use rlib::net::reqwest::Client;

pub async fn get_completion(
  engine: &str,
  prompt: &str,
  _max_tokens: u8,
  client: &Client,
) -> Result<()> {
  let url = String::from(format!(
    "http://api.openai.com/v1/engines/{}/completions",
    engine
  ));

  let res = client.post(&url).form(&[("prompt", prompt)]).send().await?;
  let body = res.text().await?;
  println!("{}", body);
  Ok(())
}

pub struct BearerToken {
  pub token: String,
}

impl std::fmt::Debug for BearerToken {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Display first characters
    write!(
      f,
      r#"Bearer {{ token: "{}*********" }}"#,
      self.token.get(0..1).ok_or(std::fmt::Error)?
    )
  }
}

impl BearerToken {
  pub fn new(token: &str) -> Self {
    Self {
      token: String::from(token),
    }
  }
}
