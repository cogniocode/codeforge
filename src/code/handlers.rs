use std::convert::Infallible;
use std::str::FromStr;

use crate::code::generator::CodeGenerator;
use crate::code::generator;
use crate::code::models::{CodeKind, CodeRequestModel};
use crate::code::models::mapping::map_to_model;
use warp::http::StatusCode;
use crate::error::ErrorModel;

pub async fn generate_code(code_request: CodeRequestModel) -> Result<impl warp::Reply, Infallible> {
    let code_kind = CodeKind::from_str(code_request.kind.as_str()).unwrap_or_default();
    let generator = generator::from_code_kind(code_kind, code_request.options.unwrap_or_default());

    let generator = match generator {
        Ok(t) => t,
        Err(_) => return Ok(warp::reply::with_status(warp::reply::json(&ErrorModel::new("Couldn't parse generation options.")), StatusCode::BAD_REQUEST))
    };

    let password = generator.generate(code_request.length);

    Ok(warp::reply::with_status(warp::reply::json(&map_to_model(&password)), StatusCode::CREATED))
}