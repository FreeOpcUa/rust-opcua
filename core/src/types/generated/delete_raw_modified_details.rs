// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteRawModifiedDetails {
    pub node_id: NodeId,
    pub is_delete_modified: Boolean,
    pub start_time: DateTime,
    pub end_time: DateTime,
}

impl BinaryEncoder<DeleteRawModifiedDetails> for DeleteRawModifiedDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.is_delete_modified.byte_len();
        size += self.start_time.byte_len();
        size += self.end_time.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.is_delete_modified.encode(stream)?;
        size += self.start_time.encode(stream)?;
        size += self.end_time.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let is_delete_modified = Boolean::decode(stream)?;
        let start_time = DateTime::decode(stream)?;
        let end_time = DateTime::decode(stream)?;
        Ok(DeleteRawModifiedDetails {
            node_id: node_id,
            is_delete_modified: is_delete_modified,
            start_time: start_time,
            end_time: end_time,
        })
    }
}