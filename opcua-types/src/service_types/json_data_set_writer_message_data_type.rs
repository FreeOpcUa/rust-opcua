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
pub struct JsonDataSetWriterMessageDataType {
    pub data_set_message_content_mask: super::enums::JsonDataSetMessageContentMask,
}
impl opcua::types::MessageInfo for JsonDataSetWriterMessageDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::JsonDataSetWriterMessageDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for JsonDataSetWriterMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_set_message_content_mask.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_set_message_content_mask.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let data_set_message_content_mask = <super::enums::JsonDataSetMessageContentMask as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            data_set_message_content_mask,
        })
    }
}