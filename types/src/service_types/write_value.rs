// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use node_id::NodeId;
use string::UAString;
use data_value::DataValue;

#[derive(Debug, Clone, PartialEq)]
pub struct WriteValue {
    pub node_id: NodeId,
    pub attribute_id: UInt32,
    pub index_range: UAString,
    pub value: DataValue,
}

impl MessageInfo for WriteValue {
    fn object_id(&self) -> ObjectId {
        ObjectId::WriteValue_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<WriteValue> for WriteValue {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size += self.value.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let attribute_id = UInt32::decode(stream)?;
        let index_range = UAString::decode(stream)?;
        let value = DataValue::decode(stream)?;
        Ok(WriteValue {
            node_id,
            attribute_id,
            index_range,
            value,
        })
    }
}
