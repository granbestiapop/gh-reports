use warp::Filter;
use crate::templates;

pub fn home() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("home")
        .and(warp::cookie("uid"))
        .and_then(templates::home_template)
        .or(default_login())
}


fn default_login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any().and_then(templates::login_template)
}