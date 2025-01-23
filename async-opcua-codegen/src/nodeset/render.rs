use std::sync::OnceLock;

use base64::Engine;
use opcua_xml::schema::ua_node_set::{LocalizedText, NodeId, QualifiedName};
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_quote, Expr};

use crate::{utils::RenderExpr, CodeGenError};

impl RenderExpr for LocalizedText {
    fn render(&self) -> Result<TokenStream, CodeGenError> {
        let locale = &self.locale.0;
        let text = &self.text;
        Ok(quote! {
            opcua::types::LocalizedText::new(#locale, #text)
        })
    }
}

static NODEID_REGEX: OnceLock<Regex> = OnceLock::new();

fn nodeid_regex() -> &'static Regex {
    NODEID_REGEX.get_or_init(|| Regex::new(r"^(ns=(?P<ns>[0-9]+);)?(?P<t>[isgb]=.+)$").unwrap())
}

pub fn split_node_id(id: &str) -> Result<(&str, &str, u16), CodeGenError> {
    let captures = nodeid_regex()
        .captures(id)
        .ok_or_else(|| CodeGenError::other(format!("Invalid nodeId: {}", id)))?;
    let namespace = if let Some(ns) = captures.name("ns") {
        ns.as_str()
            .parse::<u16>()
            .map_err(|_| CodeGenError::other(format!("Invalid nodeId: {}", id)))?
    } else {
        0
    };

    let t = captures.name("t").unwrap();
    let idf = t.as_str();
    if idf.len() < 2 {
        Err(CodeGenError::other(format!("Invalid nodeId: {}", id)))?;
    }
    let k = &idf[..2];
    let v = &idf[2..];

    Ok((k, v, namespace))
}

impl RenderExpr for NodeId {
    fn render(&self) -> Result<TokenStream, CodeGenError> {
        let id = &self.0;
        let (k, v, namespace) = split_node_id(id)?;
        // Do as much parsing as possible here, to optimize performance and get the errors as early as possible.
        let id_item: Expr = match k {
            "i=" => {
                let i = v
                    .parse::<u32>()
                    .map_err(|_| CodeGenError::other(format!("Invalid nodeId: {}", id)))?;
                parse_quote! { #i }
            }
            "s=" => {
                parse_quote! { #v }
            }
            "g=" => {
                let uuid = uuid::Uuid::parse_str(v)
                    .map_err(|e| CodeGenError::other(format!("Invalid nodeId: {}, {e}", id)))?;
                let bytes = uuid.as_bytes();
                parse_quote! { opcua::types::Uuid::from_slice(&[#(#bytes)*,]).unwrap() }
            }
            "b=" => {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(v)
                    .map_err(|e| CodeGenError::other(format!("Invalid nodeId: {}, {e}", id)))?;
                parse_quote! { opcua::types::ByteString::from(vec![#(#bytes)*,]) }
            }
            _ => return Err(CodeGenError::other(format!("Invalid nodeId: {}", id))),
        };

        let ns_item = if namespace == 0 {
            quote! { 0u16 }
        } else {
            quote! {
                ns_map.get_index(#namespace).unwrap()
            }
        };

        Ok(quote! {
            opcua::types::NodeId::new(#ns_item, #id_item)
        })
    }
}

static QUALIFIED_NAME_REGEX: OnceLock<Regex> = OnceLock::new();

fn qualified_name_regex() -> &'static Regex {
    QUALIFIED_NAME_REGEX.get_or_init(|| Regex::new(r"^((?P<ns>[0-9]+):)?(?P<name>.*)$").unwrap())
}

pub fn split_qualified_name(name: &str) -> Result<(&str, u16), CodeGenError> {
    let captures = qualified_name_regex()
        .captures(name)
        .ok_or_else(|| CodeGenError::other(format!("Invalid qualifiedname: {}", name)))?;

    let namespace = if let Some(ns) = captures.name("ns") {
        ns.as_str()
            .parse::<u16>()
            .map_err(|_| CodeGenError::other(format!("Invalid nodeId: {}", name)))?
    } else {
        0
    };

    Ok((captures.name("name").unwrap().as_str(), namespace))
}

impl RenderExpr for QualifiedName {
    fn render(&self) -> Result<TokenStream, CodeGenError> {
        let name = &self.0;
        let (name, namespace) = split_qualified_name(name)?;

        let ns_item = if namespace == 0 {
            quote! { 0u16 }
        } else {
            quote! {
                ns_map.get_index(#namespace).unwrap()
            }
        };

        Ok(quote! {
            opcua::types::QualifiedName::new(#ns_item, #name)
        })
    }
}

impl RenderExpr for Vec<u32> {
    fn render(&self) -> Result<TokenStream, CodeGenError> {
        let r = self;
        Ok(quote! {
            vec![#(#r),*]
        })
    }
}

impl RenderExpr for f64 {
    fn render(&self) -> Result<TokenStream, CodeGenError> {
        let r = self;
        Ok(quote! {
            #r
        })
    }
}
