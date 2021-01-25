use std::collections::HashMap;
use std::str::FromStr;
use std::string::ParseError;

use serde::{Deserialize, Serialize};

pub struct Code {
    pub kind: CodeKind,
    pub value: String,
    pub length: CodeLength
}

pub type CodeLength = u32;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum CodeKind {
    Traditional,
    Memorable,
}

impl Default for CodeKind {
    fn default() -> Self {
        CodeKind::Traditional
    }
}

impl FromStr for CodeKind {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "traditional" => Ok(CodeKind::Traditional),
            "memorable" => Ok(CodeKind::Memorable),
            // TODO: Review.
            _ => Ok(CodeKind::Traditional)
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CodeRequestModel {
    pub kind: String,
    pub length: u32,
    pub options: Option<HashMap<String, String>>
}

#[derive(Deserialize, Serialize)]
pub struct CodeResourceModel {
    pub kind: CodeKind,
    pub value: String,
    pub length: CodeLength
}

pub mod mapping {
    use crate::code::models::{Code, CodeResourceModel};

    pub fn map_to_model(entity: &Code) -> CodeResourceModel {
        CodeResourceModel {
            kind: entity.kind,
            value: entity.value.clone(),
            length: entity.length
        }
    }
}