use warp::Filter;

use crate::code::handlers::generate_code;

fn code_generation_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::body::json())
        .and_then(generate_code)
}

pub fn code_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("codes")
        .and(code_generation_route())
}