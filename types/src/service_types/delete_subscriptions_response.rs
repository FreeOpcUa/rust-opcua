// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::ResponseHeader;
use status_codes::StatusCode;
use basic_types::DiagnosticInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteSubscriptionsResponse {
    pub response_header: ResponseHeader,
    pub results: Option<Vec<StatusCode>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for DeleteSubscriptionsResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteSubscriptionsResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteSubscriptionsResponse> for DeleteSubscriptionsResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let results: Option<Vec<StatusCode>> = read_array(stream)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(DeleteSubscriptionsResponse {
            response_header,
            results,
            diagnostic_infos,
        })
    }
}
