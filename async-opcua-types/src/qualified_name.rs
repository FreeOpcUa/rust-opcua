// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the definition of `QualifiedName`.
use std::io::{Read, Write};

use crate::{
    encoding::{BinaryDecodable, BinaryEncodable, EncodingResult},
    string::*,
};

#[allow(unused)]
mod opcua {
    pub use crate as types;
}

/// An identifier for a error or condition that is associated with a value or an operation.
///
/// A name qualified by a namespace.
///
/// For JSON, the namespace_index is saved as "Uri" and MUST be a numeric value or it will not parse. This is
/// is in accordance with OPC UA spec that says to save the index as a numeric according to rules cut and
/// pasted from spec below:
///
/// Name   The Name component of the QualifiedName.
///
/// Uri    The _NamespaceIndexcomponent_ of the QualifiedNameencoded as a JSON number. The Urifield
///        is omitted if the NamespaceIndex equals 0. For the non-reversible form, the
///        NamespaceUriassociated with the NamespaceIndexportion of the QualifiedNameis encoded as
///        JSON string unless the NamespaceIndexis 1 or if NamespaceUriis unknown. In these cases,
///        the NamespaceIndexis encoded as a JSON number.
#[derive(PartialEq, Debug, Clone, Eq, Hash)]
#[cfg_attr(
    feature = "json",
    derive(opcua_macros::JsonEncodable, opcua_macros::JsonDecodable)
)]
pub struct QualifiedName {
    /// The namespace index
    #[cfg_attr(feature = "json", opcua(rename = "Uri"))]
    pub namespace_index: u16,
    /// The name.
    pub name: UAString,
}

impl Default for QualifiedName {
    fn default() -> Self {
        Self::null()
    }
}

impl<'a> From<&'a str> for QualifiedName {
    fn from(value: &'a str) -> Self {
        Self {
            namespace_index: 0,
            name: UAString::from(value),
        }
    }
}

impl From<&String> for QualifiedName {
    fn from(value: &String) -> Self {
        Self {
            namespace_index: 0,
            name: UAString::from(value),
        }
    }
}

impl From<String> for QualifiedName {
    fn from(value: String) -> Self {
        Self {
            namespace_index: 0,
            name: UAString::from(value),
        }
    }
}

impl BinaryEncodable for QualifiedName {
    fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
        let mut size: usize = 0;
        size += self.namespace_index.byte_len(ctx);
        size += self.name.byte_len(ctx);
        size
    }

    fn encode<S: Write + ?Sized>(
        &self,
        stream: &mut S,
        ctx: &crate::Context<'_>,
    ) -> EncodingResult<()> {
        self.namespace_index.encode(stream, ctx)?;
        self.name.encode(stream, ctx)
    }
}
impl BinaryDecodable for QualifiedName {
    fn decode<S: Read + ?Sized>(stream: &mut S, ctx: &crate::Context<'_>) -> EncodingResult<Self> {
        let namespace_index = u16::decode(stream, ctx)?;
        let name = UAString::decode(stream, ctx)?;
        Ok(QualifiedName {
            namespace_index,
            name,
        })
    }
}

impl QualifiedName {
    /// Create a new qualified name from namespace index and name.
    pub fn new<T>(namespace_index: u16, name: T) -> QualifiedName
    where
        T: Into<UAString>,
    {
        QualifiedName {
            namespace_index,
            name: name.into(),
        }
    }

    /// Create a new empty QualifiedName.
    pub fn null() -> QualifiedName {
        QualifiedName {
            namespace_index: 0,
            name: UAString::null(),
        }
    }

    /// Return `true` if this is the null QualifiedName.
    pub fn is_null(&self) -> bool {
        self.namespace_index == 0 && self.name.is_null()
    }
}
