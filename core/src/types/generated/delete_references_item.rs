// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// A request to delete a node from the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteReferencesItem {
    pub source_node_id: NodeId,
    pub reference_type_id: NodeId,
    pub is_forward: Boolean,
    pub target_node_id: ExpandedNodeId,
    pub delete_bidirectional: Boolean,
}

impl MessageInfo for DeleteReferencesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteReferencesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteReferencesItem> for DeleteReferencesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.source_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.target_node_id.byte_len();
        size += self.delete_bidirectional.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.source_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.delete_bidirectional.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let source_node_id = NodeId::decode(stream)?;
        let reference_type_id = NodeId::decode(stream)?;
        let is_forward = Boolean::decode(stream)?;
        let target_node_id = ExpandedNodeId::decode(stream)?;
        let delete_bidirectional = Boolean::decode(stream)?;
        Ok(DeleteReferencesItem {
            source_node_id: source_node_id,
            reference_type_id: reference_type_id,
            is_forward: is_forward,
            target_node_id: target_node_id,
            delete_bidirectional: delete_bidirectional,
        })
    }
}