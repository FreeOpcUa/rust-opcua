use opcua_types::Variant;
use thiserror::Error;

/// Rust OpcUa specific errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum OpcUAError {
    #[error("Received an expected variant type")]
    UnExpectedVariantType(Variant),
    #[error("The requested namespace does not exists")]
    NamespaceDoesNotExist(String),
}
