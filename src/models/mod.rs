use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLResponse {
  pub data: HashMap<String, Repository>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
  pub name: String,
  pub milestone: Milestone,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
  pub id: String,
  pub title: String,
  pub description: String,
  pub created_at: String,
  pub due_on: String,
  pub state: String,
  pub url: String,
  pub pull_requests: PullRequestConnection,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PullRequestConnection {
  pub nodes: Vec<PullRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PullRequest {
  pub id: String,
  pub state: String,
  pub title: String,
  pub permalink: String,
  pub author: Author,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
  pub login: String,
  pub avatar_url: String,
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryClient {
  pub query: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateModel {
  pub repository_name: String,
  pub title: String,
  pub description: String,
  pub created_at: String,
  pub url: String,
  pub release_tag_url: String,
  pub pull_requests: Vec<PullRequestModel>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportModel{
  pub milestones: Vec<TemplateModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestModel {
  pub title:String, 
  pub permalink: String,
  pub author: Author
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestParams {
  pub milestones: String,
  pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportOptions {
  pub token: String,
  pub milestones: Vec<MilestoneInfo>,
  pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MilestoneInfo {
  pub milestone: String,
  pub org: String,
  pub application: String,
  pub milestone_id: String,
  pub hash: String,
}