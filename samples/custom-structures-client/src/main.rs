// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! This simple OPC UA client will do the following:
//!
//! 1. Create a client configuration
//! 2. Connect to an endpoint specified by the url with security None
//! 3. Subscribe to values and loop forever printing out their values
use std::sync::Arc;

use opcua::{
    client::{custom_types::DataTypeTreeBuilder, ClientBuilder, IdentityToken, Session},
    crypto::SecurityPolicy,
    types::{
        custom::{DynamicStructure, DynamicTypeLoader},
        BrowsePath, ExpandedNodeId, MessageSecurityMode, NodeId, ObjectId, ReadValueId, StatusCode,
        TimestampsToReturn, TypeLoader, UserTokenPolicy, VariableId, Variant,
    },
};

const NAMESPACE_URI: &str = "urn:DemoServer";

struct Args {
    help: bool,
    url: String,
}

impl Args {
    pub fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();
        Ok(Args {
            help: args.contains(["-h", "--help"]),
            url: args
                .opt_value_from_str("--url")?
                .unwrap_or_else(|| String::from(DEFAULT_URL)),
        })
    }

    pub fn usage() {
        println!(
            r#"Simple Client
Usage:
  -h, --help   Show help
  --url [url]  Url to connect to (default: {})"#,
            DEFAULT_URL
        );
    }
}

const DEFAULT_URL: &str = "opc.tcp://localhost:4855";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read command line arguments
    let args = Args::parse_args()?;
    if args.help {
        Args::usage();
        return Ok(());
    }
    // Optional - enable OPC UA logging
    opcua::console_logging::init();

    // Make the client configuration
    let mut client = ClientBuilder::new()
        .application_name("Simple Client")
        .application_uri("urn:SimpleClient")
        .product_uri("urn:SimpleClient")
        .trust_server_certs(true)
        .create_sample_keypair(true)
        .session_retry_limit(3)
        .client()
        .unwrap();

    let (session, event_loop) = client
        .connect_to_matching_endpoint(
            (
                args.url.as_ref(),
                SecurityPolicy::None.to_str(),
                MessageSecurityMode::None,
                UserTokenPolicy::anonymous(),
            ),
            IdentityToken::Anonymous,
        )
        .await
        .unwrap();
    let handle = event_loop.spawn();
    session.wait_for_connection().await;

    let ns = get_namespace_idx(&session, NAMESPACE_URI).await?;
    read_structure_var(session, ns).await?;

    //TODO close session
    //handle.await.unwrap();
    Ok(())
}

async fn read_structure_var(session: Arc<Session>, ns: u16) -> Result<(), StatusCode> {
    let type_tree = DataTypeTreeBuilder::new(|f| f.namespace <= ns)
        .build(&session)
        .await
        .unwrap();

    let typ = type_tree
        .get_struct_type(&NodeId::new(ns, 3325))
        .unwrap()
        .clone();
    dbg!(&typ);
    let type_tree = Arc::new(type_tree);

    let loader = Arc::new(DynamicTypeLoader::new(type_tree.clone())) as Arc<dyn TypeLoader>;

    session.add_type_loader(loader.clone());

    let res = session
        .translate_browse_paths_to_node_ids(&[BrowsePath {
            starting_node: ObjectId::ObjectsFolder.into(),
            relative_path: (&["ErrorData".into()][..]).into(),
        }])
        .await?;
    dbg!(&res);
    let Some(target) = &res[0].targets else {
        panic!("translate browse path did not return a NodeId")
    };
    let node_id = &target[0].target_id.node_id;
    let res = session
        .read(&[node_id.into()], TimestampsToReturn::Neither, 0.0)
        .await?
        .into_iter()
        .next()
        .unwrap();
    //dbg!(&res);
    let Some(Variant::ExtensionObject(val)) = res.value else {
        panic!("Unexpected variant type");
    };
    //dbg!(&val);
    //let val: DynamicStructure = *val.into_inner_as().unwrap();
    //dbg!(val.values());
    let val: ErrorData = *val.into_inner_as().unwrap();
    dbg!(val);
    Ok(())
}

async fn get_namespace_array(
    session: &Arc<Session>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let nodeid: NodeId = VariableId::Server_NamespaceArray.into();
    let result = session
        .read(
            &[ReadValueId::from(nodeid)],
            TimestampsToReturn::Source,
            0.0,
        )
        .await?;
    if let Some(Variant::Array(array)) = &result[0].value {
        let arr = array
            .values
            .iter()
            .map(|v| {
                //TODO iterator can handle result itself!!!
                if let Variant::String(s) = v {
                    s.value().clone().unwrap_or(String::new())
                } else {
                    String::new()
                }
            })
            .collect();
        return Ok(arr);
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Path did not lead to a node {:?}", result),
    )))
}

async fn get_namespace_idx(
    session: &Arc<Session>,
    url: &str,
) -> Result<u16, Box<dyn std::error::Error>> {
    let array = get_namespace_array(session).await?;
    let idx = array.iter().position(|s| s == url).ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Namespace {} not found in {:?}", url, array),
        ))
    })?;

    Ok(idx.try_into().unwrap())
}

// the struct and enum code after that line could/should be shared with demo server
// but having it here make the example selv contained

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
//#[cfg_attr(
//feature = "json",
//derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
//)]
//#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
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
//#[cfg_attr(
//feature = "json",
//derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
//)]
//#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct ErrorData {
    message: opcua::types::UAString,
    error_id: u32,
    last_state: AxisState,
}
