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
pub struct CreateSessionResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub session_id: opcua::types::node_id::NodeId,
    pub authentication_token: opcua::types::node_id::NodeId,
    pub revised_session_timeout: f64,
    pub server_nonce: opcua::types::byte_string::ByteString,
    pub server_certificate: opcua::types::byte_string::ByteString,
    pub server_endpoints: Option<Vec<super::endpoint_description::EndpointDescription>>,
    pub server_software_certificates:
        Option<Vec<super::signed_software_certificate::SignedSoftwareCertificate>>,
    pub server_signature: super::signature_data::SignatureData,
    pub max_request_message_size: u32,
}
impl opcua::types::MessageInfo for CreateSessionResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSessionResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSessionResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSessionResponse_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::CreateSessionResponse
    }
}