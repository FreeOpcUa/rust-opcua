// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use byte_string::ByteString;

/// This abstract Structured DataType is the base DataType for all DataTypes representing a bit mask.
#[derive(Debug, Clone, PartialEq)]
pub struct OptionSet {
    pub value: ByteString,
    pub valid_bits: ByteString,
}

impl MessageInfo for OptionSet {
    fn object_id(&self) -> ObjectId {
        ObjectId::OptionSet_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<OptionSet> for OptionSet {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.value.byte_len();
        size += self.valid_bits.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.value.encode(stream)?;
        size += self.valid_bits.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let value = ByteString::decode(stream)?;
        let valid_bits = ByteString::decode(stream)?;
        Ok(OptionSet {
            value,
            valid_bits,
        })
    }
}
