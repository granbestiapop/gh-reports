use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GithubAuthResponse {
	pub access_token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GithubAuthRequest {
	pub code: String,
	pub client_id: String,
	pub client_secret: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GithubCallbackRequest {
	pub code: String,
}