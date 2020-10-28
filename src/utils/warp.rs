use std::convert::Infallible;
use warp::Filter;

pub fn with<T: Clone + Send>(obj: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || obj.clone())
}
