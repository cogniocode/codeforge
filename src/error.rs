use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorModel {
    message: String
}

impl ErrorModel {
    pub fn new(message: &str) -> ErrorModel {
        ErrorModel {
            message: message.to_string()
        }
    }
}