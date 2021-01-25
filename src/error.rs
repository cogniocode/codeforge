use serde::Serialize;
use std::str::FromStr;
use std::convert::Infallible;

#[derive(Serialize)]
pub struct ErrorModel {
    message: String
}

impl FromStr for ErrorModel {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ErrorModel { message: s.to_string() })
    }
}