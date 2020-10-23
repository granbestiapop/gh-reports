use crate::services;
use crate::services::factory::SharedContext;
use crate::utils;
use crate::models;
use crate::templates;
use warp::Filter;

pub fn reports(
    context: SharedContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("reports"))
        .and(utils::warp::with(context))
        .and(parse_reports_filter())
        .and_then(fetch_reports)
        .and_then(templates::render_template)
}

pub fn parse_reports_filter(
) -> impl Filter<Extract = (models::ReportOptions,), Error = warp::reject::Rejection> + Copy {
    warp::any()
        .and(warp::query::<models::RequestParams>())
        .and(warp::cookie("uid"))
        .map(services::milestone::parse_params)
}

async fn fetch_reports(
    context: SharedContext,
    params: models::ReportOptions,
) -> Result<templates::HtmlReportTemplate, warp::Rejection> {
    let title = params.clone().title;
    let data = context.milestone_service.get_milestone(params.clone()).await.unwrap();
    let template = templates::HtmlReportTemplate {
        title: title.clone(),
        milestones: data.milestones,
    };
    return Ok(template);
}