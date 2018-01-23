// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use string::UAString;
use basic_types::LocalizedText;
use service_types::enums::ApplicationType;

/// Describes an application and how to find it.
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationDescription {
    pub application_uri: UAString,
    pub product_uri: UAString,
    pub application_name: LocalizedText,
    pub application_type: ApplicationType,
    pub gateway_server_uri: UAString,
    pub discovery_profile_uri: UAString,
    pub discovery_urls: Option<Vec<UAString>>,
}

impl MessageInfo for ApplicationDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::ApplicationDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ApplicationDescription> for ApplicationDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.application_uri.byte_len();
        size += self.product_uri.byte_len();
        size += self.application_name.byte_len();
        size += self.application_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += self.discovery_profile_uri.byte_len();
        size += byte_len_array(&self.discovery_urls);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.application_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += self.application_name.encode(stream)?;
        size += self.application_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += self.discovery_profile_uri.encode(stream)?;
        size += write_array(stream, &self.discovery_urls)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let application_uri = UAString::decode(stream)?;
        let product_uri = UAString::decode(stream)?;
        let application_name = LocalizedText::decode(stream)?;
        let application_type = ApplicationType::decode(stream)?;
        let gateway_server_uri = UAString::decode(stream)?;
        let discovery_profile_uri = UAString::decode(stream)?;
        let discovery_urls: Option<Vec<UAString>> = read_array(stream)?;
        Ok(ApplicationDescription {
            application_uri,
            product_uri,
            application_name,
            application_type,
            gateway_server_uri,
            discovery_profile_uri,
            discovery_urls,
        })
    }
}
