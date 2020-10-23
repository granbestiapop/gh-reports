
use crate::clients::{GithubAuthRequest, GithubClient};

pub struct AuthService {
  client: GithubClient,
  client_secret: String,
  client_id: String,
}

impl AuthService {
  pub fn new() -> Self {
    println!("new auth service");
    Self {
      client: GithubClient::new(),
      client_secret: std::env::var("GITHUB_CLIENT_SECRET").expect("Set github client secret"),
      client_id: std::env::var("GITHUB_CLIENT_ID").expect("Set github client secret"),
    }
  }

  pub async fn auth(
    &self,
    parameters: &GithubAuthRequest,
  ) -> Result<String, Box<dyn std::error::Error>> {
    let response = self.client.get_access_token(parameters).await?;
    Ok(response.access_token)
  }

  pub fn create_request(&self, code: String) -> GithubAuthRequest {
    GithubAuthRequest {
      code: code,
      client_id: self.client_id.clone(),
      client_secret: self.client_secret.clone(),
    }
  }
}
