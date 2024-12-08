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
pub struct FindServersOnNetworkRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub starting_record_id: u32,
    pub max_records_to_return: u32,
    pub server_capability_filter: Option<Vec<opcua::types::string::UAString>>,
}
impl opcua::types::MessageInfo for FindServersOnNetworkRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FindServersOnNetworkRequest_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::FindServersOnNetworkRequest
    }
}
