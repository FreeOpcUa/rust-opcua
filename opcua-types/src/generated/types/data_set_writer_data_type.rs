// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
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
pub struct DataSetWriterDataType {
    pub name: opcua::types::string::UAString,
    pub enabled: bool,
    pub data_set_writer_id: u16,
    pub data_set_field_content_mask: super::enums::DataSetFieldContentMask,
    pub key_frame_count: u32,
    pub data_set_name: opcua::types::string::UAString,
    pub data_set_writer_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub transport_settings: opcua::types::extension_object::ExtensionObject,
    pub message_settings: opcua::types::extension_object::ExtensionObject,
}
impl opcua::types::MessageInfo for DataSetWriterDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetWriterDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetWriterDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetWriterDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::DataSetWriterDataType
    }
}
