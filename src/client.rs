use crate::models;
use crate::templates;

/**
 * Fetch data from github graphql service
 */
pub async fn fetch_repo_data(
	options: models::ReportOptions,
) -> Result<models::GraphQLResponse, reqwest::Error> {
	let client = reqwest::Client::new();
	let query = get_query(&options);
	let model = models::QueryClient { query: query };
	let token_header = format!("bearer {}", options.token);

	let response: models::GraphQLResponse = client
		.post("https://api.github.com/graphql")
		.header(reqwest::header::USER_AGENT, "rust_kalli")
		.header(reqwest::header::AUTHORIZATION, token_header)
		.body(serde_json::to_string(&model).unwrap())
		.send()
		.await?
		.json()
		.await?;
	//println!("body = {:?}", response);
	Ok(response)
}

fn get_query(options: &models::ReportOptions) -> String {
	templates::TEMPLATE_ENGINE
		.render("query", &json!({"milestones": options.milestones}))
		.unwrap_or_else(|err| err.to_string())
}
