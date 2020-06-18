use regex::Regex;
use hex;
use crate::models;
use crate::client;

pub async fn get_milestone(params: models::ReportOptions) -> Result<Vec<models::TemplateModel>, reqwest::Error> {
    let response = client::fetch_repo_data(params).await?;
    Ok(get_milestone_info(&response))
}

fn get_milestone_info(response: &models::GraphQLResponse) -> Vec<models::TemplateModel>{
    let templates:Vec<models::TemplateModel> = response.data.iter().map(|(_, repository)| repository)
        .map(|repository|{
            let pull_requests = repository.milestone.pull_requests.nodes.iter().map(|pr| models::PullRequestModel{
                title: pr.title.to_owned(),
                permalink: pr.permalink.to_owned(),
                author: pr.author.clone()
            }).collect();
            models::TemplateModel{
                repository_name: repository.name.to_owned(),
                title: repository.milestone.title.to_owned(),
                description: repository.milestone.description.to_owned(),
                created_at:repository.milestone.created_at.to_owned(),
                url: repository.milestone.url.to_owned(),
                pull_requests: pull_requests,
                release_tag_url: "".to_string(), //TODO mobile
            }
        })
        .collect();
    templates
}


pub fn parse_params(params: models::RequestParams) -> models::ReportOptions{
    let re = Regex::new(r"https://github.com/(.*)/(.*)/milestone/(.*)").unwrap();
    let milestones: Vec<&str> = params.milestones.split(",").collect();
    let milestones_options = milestones.iter().map(|m|{
        let captures= re.captures(m).unwrap();
        models::MilestoneInfo{
            org: captures[1].to_owned(),
            application: captures[2].to_owned(),
            milestone_id: captures[3].to_owned(),
            milestone: m.to_string(),
            hash: format!("m_{}", hex::encode(m)),
        }
    }).collect();

    models::ReportOptions{
        token: params.token,
        milestones: milestones_options,
    }
}