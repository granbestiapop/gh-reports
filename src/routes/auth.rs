
use crate::services::factory::SharedContext;
use crate::utils;
use crate::clients::responses::{GithubCallbackRequest};

use warp::Filter;

pub fn auth(
    context: SharedContext,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("reports" / "callback")
        .and(utils::warp::with(context))
        .and(warp::header(warp::http::header::HOST.as_str()))
        .and(warp::query::<GithubCallbackRequest>())
        .and_then(github_callback)
        .and_then(redirect)
}

async fn github_callback(
    context: SharedContext,
    host: String,
    request: GithubCallbackRequest,
) -> Result<RedirectModel, warp::Rejection> {
    let request = context.auth_service.create_request(request.code);
    match context.auth_service.auth(&request).await {
        Ok(token) => Ok(RedirectModel {
            token: token,
            host: host,
        }),
        Err(_e) => Err(warp::reject::not_found()),
    }
}

#[derive(Debug, Clone)]
struct RedirectModel {
    token: String,
    host: String,
}

async fn redirect(redirect_model: RedirectModel) -> Result<impl warp::Reply, warp::Rejection> {
    let token = redirect_model.token.as_str();
    let host = redirect_model.host.as_str();
    let reply = utils::http::redirect_with_cookie("/home", "uid", token, host);
    Ok(reply)
}
