use warp::Filter;

mod client;
mod models;
mod services;
mod templates;

use templates::WithTemplate;

#[macro_use]
extern crate serde_json;

#[tokio::main]
async fn main() {
    let hello = warp::get()
        .and(warp::path("reports"))
        .and(warp::query::<models::RequestParams>())
        .map(services::milestone::parse_params)
        .and_then(create_template)
        .or(warp::path!("hello" / String).map(|name| format!("Hello, {}!", name)));
    warp::serve(hello).run(([0, 0, 0, 0], 3333)).await;
}

async fn create_template(
    params: models::ReportOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let templates = services::milestone::get_milestone(params).await.unwrap();
    let template = WithTemplate {
        name: "template",
        value: json!({ "templates": templates }),
    };
    templates::render(template)
}

/* ARC example for database inject
fn with_template(hs: Arc<Handlebars<'_>>) -> impl Filter<Extract = (Arc<Handlebars<'_>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || hs.clone())
}*/
