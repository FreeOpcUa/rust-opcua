// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0

//!  Rust OpcUa specific errors

use thiserror::Error;

use crate::{StatusCode, VariantScalarTypeId};

/// Rust OpcUa specific errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum OpcUaError {
    #[error("Received an unexpected variant type")]
    UnexpectedVariantType {
        variant_id: Option<VariantScalarTypeId>,
        message: String,
    },
    #[error("The requested namespace does not exists")]
    NamespaceDoesNotExist(String),
    #[error("Namespace is out of range of a u16.")]
    NamespaceOutOfRange,
    #[error("Supplied node resolver was unable to resolve a reference type.")]
    UnresolvedReferenceType,
    #[error("Path does not match a node.")]
    NoMatch,
    #[error("Path segment is unusually long and has been rejected.")]
    PathSegmentTooLong,
    #[error("Number of elements in relative path is too large.")]
    TooManyElementsInPath,
    #[error("Request returned a StatusCode Error.")]
    StatusCodeError(StatusCode),
    #[error("Generic Error.")]
    Error(crate::Error),
}

impl From<StatusCode> for OpcUaError {
    fn from(value: StatusCode) -> Self {
        OpcUaError::StatusCodeError(value)
    }
}

impl From<crate::Error> for OpcUaError {
    fn from(value: crate::Error) -> Self {
        OpcUaError::Error(value)
    }
}
