use crate::Result;
pub struct Client {
  prompt: String,
}

impl Client {
  pub fn new(key: &str) -> Self {
    Client {
      prompt: "".to_string(),
    }
  }

  pub async fn get_completion(self, prompt: &str) -> Result<&str> {
    println!("{}", prompt);
    Ok(prompt)
  }
}
