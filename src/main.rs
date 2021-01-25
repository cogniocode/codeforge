use warp::Filter;

use crate::code::routes::code_routes;

mod code;
mod error;
mod config;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = config::init();

    let port = config.get("port").unwrap_or(8080);

    warp::serve(routes()).run(([127, 0, 0, 1], port)).await;
}

fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    code_routes()
}
