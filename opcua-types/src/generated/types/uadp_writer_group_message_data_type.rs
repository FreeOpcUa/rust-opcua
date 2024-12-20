// This file was autogenerated from schemas/1.05/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct UadpWriterGroupMessageDataType {
    pub group_version: u32,
    pub data_set_ordering: super::enums::DataSetOrderingType,
    pub network_message_content_mask: super::enums::UadpNetworkMessageContentMask,
    pub sampling_offset: f64,
    pub publishing_offset: Option<Vec<f64>>,
}
impl opcua::types::MessageInfo for UadpWriterGroupMessageDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UadpWriterGroupMessageDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UadpWriterGroupMessageDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UadpWriterGroupMessageDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::UadpWriterGroupMessageDataType
    }
}
