use askama::Template;
use crate::models;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate{}


#[derive(Template)]
#[template(path = "html_report.html")]
pub struct HtmlReportTemplate {
  pub title: String,
  pub milestones: Vec<models::TemplateModel>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate{}

#[derive(Template)]
#[template(path = "query.html")]
pub struct QueryTemplate{
  pub milestones: Vec<models::MilestoneInfo>
}


pub async fn render_template<T: Template>(template: T) -> Result<impl warp::Reply, warp::Rejection> {
    let string = template.render().unwrap();
    Ok(warp::reply::html(string))
}

pub async fn home_template(_: String) -> Result<impl warp::Reply, warp::Rejection> {
    render_template(HomeTemplate {}).await
}

pub async fn login_template() -> Result<impl warp::Reply, warp::Rejection> {
    render_template(LoginTemplate {}).await
}