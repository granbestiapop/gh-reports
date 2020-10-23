use handlebars::Handlebars;
use lazy_static::lazy_static;
use askama::Template;
use crate::models;

lazy_static! {
  pub static ref TEMPLATE_ENGINE: Handlebars<'static> = {
    let mut hb = Handlebars::new();
    hb.register_template_file("query", "./src/templates/query.hbs")
      .unwrap();
    hb.register_template_file("template", "./src/templates/html.hbs")
      .unwrap();
    println!("Creating template engine!");
    hb
  };
}


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