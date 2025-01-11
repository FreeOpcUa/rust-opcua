use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("Failed to load XML: {0}")]
    XML(#[from] opcua_xml::XmlError),
    #[error("Missing required field: {0}")]
    MissingRequiredValue(&'static str),
    #[error("Wrong format on field. Expected {0}, got {1}")]
    WrongFormat(String, String),
    #[error("Failed to parse {0} as integer.")]
    ParseInt(String, ParseIntError),
    #[error("Failed to parse {0} as bool.")]
    ParseBool(String, ParseBoolError),
    #[error("Failed to parse {0} as float.")]
    ParseFloat(String, ParseFloatError),
    #[error("{0}")]
    Other(String),
    #[error("Failed to generate code: {0}")]
    Syn(#[from] syn::Error),
    #[error("{0}: {1}")]
    Io(String, std::io::Error),
}

impl From<ParseIntError> for CodeGenError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt("content".to_owned(), value)
    }
}

impl From<ParseBoolError> for CodeGenError {
    fn from(value: ParseBoolError) -> Self {
        Self::ParseBool("content".to_owned(), value)
    }
}

impl From<ParseFloatError> for CodeGenError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloat("content".to_owned(), value)
    }
}

impl CodeGenError {
    pub fn io(msg: &str, e: std::io::Error) -> Self {
        Self::Io(msg.to_owned(), e)
    }
}
