// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0

//!  Rust OpcUa specific errors

use opcua_types::VariantScalarTypeId;
use thiserror::Error;

/// Rust OpcUa specific errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum OpcUAError {
    #[error("Received an unexpected variant type")]
    UnExpectedVariantType(Option<VariantScalarTypeId>),
    #[error("The requested namespace does not exists")]
    NamespaceDoesNotExist(String),
}
