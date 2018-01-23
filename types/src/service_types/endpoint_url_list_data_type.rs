// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use string::UAString;

#[derive(Debug, Clone, PartialEq)]
pub struct EndpointUrlListDataType {
    pub endpoint_url_list: Option<Vec<UAString>>,
}

impl MessageInfo for EndpointUrlListDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::EndpointUrlListDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EndpointUrlListDataType> for EndpointUrlListDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.endpoint_url_list);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.endpoint_url_list)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let endpoint_url_list: Option<Vec<UAString>> = read_array(stream)?;
        Ok(EndpointUrlListDataType {
            endpoint_url_list,
        })
    }
}
