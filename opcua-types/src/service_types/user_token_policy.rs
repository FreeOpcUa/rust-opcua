// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
pub struct UserTokenPolicy {
    pub policy_id: opcua::types::string::UAString,
    pub token_type: super::enums::UserTokenType,
    pub issued_token_type: opcua::types::string::UAString,
    pub issuer_endpoint_url: opcua::types::string::UAString,
    pub security_policy_uri: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for UserTokenPolicy {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UserTokenPolicy_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for UserTokenPolicy {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.policy_id.byte_len();
        size += self.token_type.byte_len();
        size += self.issued_token_type.byte_len();
        size += self.issuer_endpoint_url.byte_len();
        size += self.security_policy_uri.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.policy_id.encode(stream)?;
        size += self.token_type.encode(stream)?;
        size += self.issued_token_type.encode(stream)?;
        size += self.issuer_endpoint_url.encode(stream)?;
        size += self.security_policy_uri.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let policy_id = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let token_type = <super::enums::UserTokenType as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let issued_token_type = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let issuer_endpoint_url = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_policy_uri = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            policy_id,
            token_type,
            issued_token_type,
            issuer_endpoint_url,
            security_policy_uri,
        })
    }
}