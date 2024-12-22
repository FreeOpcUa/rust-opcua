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
pub struct DeleteReferencesItem {
    pub source_node_id: opcua::types::node_id::NodeId,
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub is_forward: bool,
    pub target_node_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub delete_bidirectional: bool,
}
impl opcua::types::MessageInfo for DeleteReferencesItem {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteReferencesItem_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteReferencesItem_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteReferencesItem_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::DeleteReferencesItem
    }
}
