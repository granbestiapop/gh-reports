use crate::models;
use crate::templates;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct GithubClient {
	client: reqwest::Client,
}

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

impl GithubClient {
	pub fn new() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}

	/**
	 * Fetch data from github graphql service
	 */
	pub async fn fetch_repo_data(
		&self,
		options: models::ReportOptions,
	) -> Result<models::GraphQLResponse, reqwest::Error> {
		let query = get_query(&options);
		let model = models::QueryClient { query: query };
		let token_header = format!("bearer {}", options.token);

		let response: models::GraphQLResponse = self
			.client
			.post("https://api.github.com/graphql")
			.header(reqwest::header::USER_AGENT, "rust_kalli")
			.header(reqwest::header::AUTHORIZATION, token_header)
			.body(serde_json::to_string(&model).unwrap())
			.send()
			.await?
			.json()
			.await?;
		println!("body = {:?}", response);
		Ok(response)
	}

	pub async fn get_access_token(
		&self,
		parameters: &GithubAuthRequest,
	) -> Result<GithubAuthResponse, reqwest::Error> {
		let response: GithubAuthResponse = self
			.client
			.post("https://github.com/login/oauth/access_token")
			.header(reqwest::header::CONTENT_TYPE, "application/json")
			.header(reqwest::header::ACCEPT, "application/json")
			.body(serde_json::to_string(parameters).unwrap())
			.send()
			.await?
			.json()
			.await?;
		println!("body = {:?}", response);
		Ok(response)
	}
}

fn get_query(options: &models::ReportOptions) -> String {
	templates::TEMPLATE_ENGINE
		.render("query", &json!({"milestones": options.milestones}))
		.unwrap_or_else(|err| err.to_string())
}