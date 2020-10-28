use cookie::Cookie;
use warp::http::header::{LOCATION, SET_COOKIE};
use warp::http::Response;

pub fn redirect_with_cookie(
    location: &str,
    cookie_key: &str,
    cookie_value: &str,
    host: &str,
) -> Response<String> {
    let domain = reqwest::Url::parse(host)
        .unwrap()
        .host_str()
        .unwrap_or("")
        .to_string();
    let cookie = Cookie::build(cookie_key, cookie_value)
        .domain(domain)
        .path("/")
        .finish()
        .to_string();
    Response::builder()
        .header(LOCATION, location)
        .header(
            SET_COOKIE,
            cookie, //format!("{}={}; Path=/; Domain={}", cookie_key, cookie_value, domain),
        )
        .status(warp::http::StatusCode::MOVED_PERMANENTLY)
        .body("".to_string())
        .unwrap()
}
