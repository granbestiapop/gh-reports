use crate::models;
use crate::templates;
use super::responses::*;

use serde_json::json;

#[derive(Clone)]
pub struct GithubClient {
	client: reqwest::Client,
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