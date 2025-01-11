// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0

//!  Rust OpcUa specific errors

use thiserror::Error;

use crate::VariantScalarTypeId;

/// Rust OpcUa specific errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum OpcUAError {
    #[error("Received an unexpected variant type")]
    UnexpectedVariantType {
        variant_id: Option<VariantScalarTypeId>,
        message: String,
    },
    #[error("The requested namespace does not exists")]
    NamespaceDoesNotExist(String),
}
