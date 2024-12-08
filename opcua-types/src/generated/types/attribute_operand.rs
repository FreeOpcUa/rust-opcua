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
pub struct AttributeOperand {
    pub node_id: opcua::types::node_id::NodeId,
    pub alias: opcua::types::string::UAString,
    pub browse_path: super::relative_path::RelativePath,
    pub attribute_id: u32,
    pub index_range: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for AttributeOperand {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AttributeOperand_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AttributeOperand_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AttributeOperand_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::AttributeOperand
    }
}
