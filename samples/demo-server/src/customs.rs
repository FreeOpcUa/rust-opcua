use std::sync::Arc;

use opcua::{
    nodes::{DataTypeBuilder, ObjectBuilder, ReferenceDirection, VariableBuilder},
    server::node_manager::memory::SimpleNodeManager,
    types::{
        DataTypeDefinition, DataTypeId, EnumDefinition, EnumField, ExpandedNodeId, ExtensionObject,
        NodeId, ObjectId, ObjectTypeId, ReferenceTypeId, StructureDefinition, StructureField,
        StructureType,
    },
};

use crate::NAMESPACE_URI;

const STRUCT_ENC_TYPE_ID: u32 = 3324;
const STRUCT_DATA_TYPE_ID: u32 = 3325;
const ENUM_DATA_TYPE_ID: u32 = 3326;

pub fn add_custom_types(nm: Arc<SimpleNodeManager>, ns: u16) {
    let enum_id = add_enum_data_type(&nm, ns);
    let struct_id = add_custom_data_type(&nm, ns, &enum_id);

    let addr = nm.address_space();
    let mut addr = addr.write();
    let error_node = NodeId::next_numeric(ns);
    let error_data = ErrorData::new("No Error", 98, AxisState::Idle);
    VariableBuilder::new(&error_node, "ErrorData", "ErrorData")
        .organized_by(ObjectId::ObjectsFolder)
        .data_type(struct_id.clone())
        .value(ExtensionObject::new(error_data))
        .insert(&mut *addr);
}

fn enum_field(name: &str, value: i64) -> EnumField {
    EnumField {
        name: name.into(),
        description: name.into(),
        display_name: name.into(),
        value,
    }
}

fn add_enum_data_type(nm: &Arc<SimpleNodeManager>, ns: u16) -> NodeId {
    let mut addr = nm.address_space().write();

    let id = NodeId::new(ns, ENUM_DATA_TYPE_ID);

    let type_def = DataTypeDefinition::Enum(EnumDefinition {
        fields: Some(vec![
            enum_field("Disabled", 1),
            enum_field("Enabled", 2),
            enum_field("Idle", 3),
            enum_field("MoveAbs", 4),
            enum_field("Error", 5),
        ]),
    });
    DataTypeBuilder::new(&id, "AxisState", "AxisState")
        .subtype_of(DataTypeId::Enumeration)
        .data_type_definition(type_def)
        .insert(&mut *addr);

    id
}

fn add_encoding(nm: &SimpleNodeManager, ns: u16, struct_id: &NodeId) -> NodeId {
    let mut addr = nm.address_space().write();
    let id = NodeId::new(ns, STRUCT_ENC_TYPE_ID);
    ObjectBuilder::new(&id, "Default Binary", "Default Binary")
        .reference(
            struct_id,
            ReferenceTypeId::HasEncoding,
            ReferenceDirection::Inverse,
        )
        .has_type_definition(ObjectTypeId::DataTypeEncodingType)
        .insert(&mut *addr);
    id
}

fn add_custom_data_type(nm: &SimpleNodeManager, ns: u16, e_state_id: &NodeId) -> NodeId {
    let struct_id = NodeId::new(ns, STRUCT_DATA_TYPE_ID);
    let enc_id = add_encoding(nm, ns, &struct_id);

    let type_def = DataTypeDefinition::Structure(StructureDefinition {
        default_encoding_id: NodeId::new(ns, STRUCT_ENC_TYPE_ID),
        base_data_type: DataTypeId::Structure.into(),
        structure_type: StructureType::Structure,
        fields: Some(vec![
            StructureField {
                name: "sErrorMessage".into(),
                data_type: DataTypeId::String.into(),
                value_rank: -1,
                ..Default::default()
            },
            StructureField {
                name: "nErrorID".into(),
                data_type: DataTypeId::UInt32.into(),
                value_rank: -1,
                ..Default::default()
            },
            StructureField {
                name: "eLastState".into(),
                data_type: e_state_id.clone(),
                value_rank: -1,
                ..Default::default()
            },
        ]),
    });
    let mut addr = nm.address_space().write();
    DataTypeBuilder::new(&struct_id, "ErrorData", "ErrorData")
        .subtype_of(DataTypeId::Structure)
        .data_type_definition(type_def)
        .reference(
            enc_id,
            ReferenceTypeId::HasEncoding,
            ReferenceDirection::Forward,
        )
        .insert(&mut *addr);

    struct_id
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
#[repr(i32)]
pub enum AxisState {
    #[default]
    Disabled = 1i32,
    Enabled = 2i32,
    Idle = 3i32,
    MoveAbs = 4i32,
    Error = 5i32,
}

#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct ErrorData {
    message: opcua::types::UAString,
    error_id: u32,
    last_state: AxisState,
}

impl ErrorData {
    pub fn new(msg: &str, error_id: u32, last_state: AxisState) -> Self {
        Self {
            message: msg.into(),
            error_id,
            last_state,
        }
    }
}

impl opcua::types::ExpandedMessageInfo for ErrorData {
    fn full_type_id(&self) -> opcua::types::ExpandedNodeId {
        ExpandedNodeId {
            node_id: NodeId::new(0, STRUCT_ENC_TYPE_ID),
            namespace_uri: NAMESPACE_URI.into(),
            server_index: 0,
        }
    }

    fn full_json_type_id(&self) -> opcua::types::ExpandedNodeId {
        todo!()
    }

    fn full_xml_type_id(&self) -> opcua::types::ExpandedNodeId {
        todo!()
    }

    fn full_data_type_id(&self) -> opcua::types::ExpandedNodeId {
        ExpandedNodeId {
            node_id: NodeId::new(0, STRUCT_DATA_TYPE_ID),
            namespace_uri: NAMESPACE_URI.into(),
            server_index: 0,
        }
    }
}