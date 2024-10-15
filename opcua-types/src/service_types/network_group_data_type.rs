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
#[derive(Default)]
pub struct NetworkGroupDataType {
    pub server_uri: opcua::types::string::UAString,
    pub network_paths: Option<
        Vec<super::endpoint_url_list_data_type::EndpointUrlListDataType>,
    >,
}
impl opcua::types::MessageInfo for NetworkGroupDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::NetworkGroupDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for NetworkGroupDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.server_uri.byte_len();
        size += self.network_paths.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.server_uri.encode(stream)?;
        size += self.network_paths.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let server_uri = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let network_paths = <Option<
            Vec<super::endpoint_url_list_data_type::EndpointUrlListDataType>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { server_uri, network_paths })
    }
}