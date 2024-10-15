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
pub struct QueryNextResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub query_data_sets: Option<Vec<super::query_data_set::QueryDataSet>>,
    pub revised_continuation_point: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for QueryNextResponse {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QueryNextResponse_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for QueryNextResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.query_data_sets.byte_len();
        size += self.revised_continuation_point.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.query_data_sets.encode(stream)?;
        size += self.revised_continuation_point.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let response_header = <opcua::types::response_header::ResponseHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = response_header.request_handle;
        let query_data_sets = <Option<
            Vec<super::query_data_set::QueryDataSet>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let revised_continuation_point = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            response_header,
            query_data_sets,
            revised_continuation_point,
        })
    }
}