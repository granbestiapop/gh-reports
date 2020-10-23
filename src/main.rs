use warp::Filter;

mod routes;
mod services;
mod utils;
mod clients;
mod models;
mod templates;

#[tokio::main]
async fn main() {
    let context = services::factory::create_context();
    let app = routes::health::route()
        .or(routes::reports::reports(context.clone()))
        .or(routes::auth::auth(context.clone()))
        .or(routes::home::home());

    warp::serve(app).run(([0, 0, 0, 0], 3333)).await;
}
