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
pub struct ModifySubscriptionResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub revised_publishing_interval: f64,
    pub revised_lifetime_count: u32,
    pub revised_max_keep_alive_count: u32,
}
impl opcua::types::MessageInfo for ModifySubscriptionResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModifySubscriptionResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModifySubscriptionResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModifySubscriptionResponse_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::ModifySubscriptionResponse
    }
}