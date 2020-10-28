use warp::Filter;

pub fn route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health").map(|| "ok")
}
