// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::ResponseHeader;
use node_id::NodeId;
use byte_string::ByteString;
use service_types::EndpointDescription;
use service_types::SignedSoftwareCertificate;
use service_types::SignatureData;

/// Creates a new session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSessionResponse {
    pub response_header: ResponseHeader,
    pub session_id: NodeId,
    pub authentication_token: NodeId,
    pub revised_session_timeout: Double,
    pub server_nonce: ByteString,
    pub server_certificate: ByteString,
    pub server_endpoints: Option<Vec<EndpointDescription>>,
    pub server_software_certificates: Option<Vec<SignedSoftwareCertificate>>,
    pub server_signature: SignatureData,
    pub max_request_message_size: UInt32,
}

impl MessageInfo for CreateSessionResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSessionResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSessionResponse> for CreateSessionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.session_id.byte_len();
        size += self.authentication_token.byte_len();
        size += self.revised_session_timeout.byte_len();
        size += self.server_nonce.byte_len();
        size += self.server_certificate.byte_len();
        size += byte_len_array(&self.server_endpoints);
        size += byte_len_array(&self.server_software_certificates);
        size += self.server_signature.byte_len();
        size += self.max_request_message_size.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.session_id.encode(stream)?;
        size += self.authentication_token.encode(stream)?;
        size += self.revised_session_timeout.encode(stream)?;
        size += self.server_nonce.encode(stream)?;
        size += self.server_certificate.encode(stream)?;
        size += write_array(stream, &self.server_endpoints)?;
        size += write_array(stream, &self.server_software_certificates)?;
        size += self.server_signature.encode(stream)?;
        size += self.max_request_message_size.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let session_id = NodeId::decode(stream)?;
        let authentication_token = NodeId::decode(stream)?;
        let revised_session_timeout = Double::decode(stream)?;
        let server_nonce = ByteString::decode(stream)?;
        let server_certificate = ByteString::decode(stream)?;
        let server_endpoints: Option<Vec<EndpointDescription>> = read_array(stream)?;
        let server_software_certificates: Option<Vec<SignedSoftwareCertificate>> = read_array(stream)?;
        let server_signature = SignatureData::decode(stream)?;
        let max_request_message_size = UInt32::decode(stream)?;
        Ok(CreateSessionResponse {
            response_header,
            session_id,
            authentication_token,
            revised_session_timeout,
            server_nonce,
            server_certificate,
            server_endpoints,
            server_software_certificates,
            server_signature,
            max_request_message_size,
        })
    }
}
