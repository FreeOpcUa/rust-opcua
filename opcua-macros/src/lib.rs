#![warn(missing_docs)]

//! Crate containing various procedural macros used by rust OPC-UA.

mod encoding;
mod events;
mod utils;

use encoding::{generate_encoding_impl, EncodingToImpl};
use events::{derive_event_field_inner, derive_event_inner};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Event, attributes(opcua))]
/// Derive the `Event` trait. This will also generate
/// an implementation of the `EventField` trait.
///
/// The event struct must have an attribute `opcua` containing
/// the _identifier_ of the node ID, as well as the namespace URI of the event type.
///
/// It must also have a field `base` with a different event type, which may be
/// the `BaseEventType`, and a field `own_namespace_index` storing the namespace index of
/// the event.
///
/// By default, fields will be given `PascalCase` names, you may use `opcua[rename = ...]`
/// to rename individual fields.
///
/// # Example
///
/// ```ignore
/// #[derive(Event)]
/// #[opcua(identifier = "s=myevent", namespace = "uri:my:namespace")]
/// struct MyEvent {
///     base: BaseEventType,
///     own_namespace_index: u16,
///     
///     #[opcua(rename = "my-field")]
///     my_field: f32,
///     my_other_field: EUInformation,
///     #[opcua(ignore)]
///     ignored: i32,
/// }
/// ```
pub fn derive_event(item: TokenStream) -> TokenStream {
    match derive_event_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(EventField, attributes(opcua))]
/// Derive the `EventField` trait.
///
/// The event field may have a field `base`, which unless renamed will
/// be used as the base type for this field.
///
/// By default, fields will be given `PascalCase` names, you may use `opcua[rename = ...]`
/// to rename individual fields.
///
/// # Example
///
/// ```ignore
/// #[derive(EventField)]
/// struct MyEventField {
///     float: f32,
/// }
/// ```
pub fn derive_event_field(item: TokenStream) -> TokenStream {
    match derive_event_field_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(feature = "xml")]
#[proc_macro_derive(FromXml, attributes(opcua))]
/// Derive the `FromXml` trait on this struct, creating a conversion from
/// NodeSet2 XML files.
///
/// All fields must be marked with `opcua(ignore)` or implement `FromXml`.
pub fn derive_from_xml(item: TokenStream) -> TokenStream {
    match generate_encoding_impl(parse_macro_input!(item), EncodingToImpl::FromXml) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(feature = "json")]
#[proc_macro_derive(JsonEncodable, attributes(opcua))]
/// Derive the `JsonEncodable` trait on this struct, creating code
/// to write the struct to a JSON stream on OPC-UA reversible encoding.
///
/// All fields must be marked with `opcua(ignore)` or implement `JsonEncodable`.
pub fn derive_json_encodable(item: TokenStream) -> TokenStream {
    match generate_encoding_impl(parse_macro_input!(item), EncodingToImpl::JsonEncode) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(feature = "json")]
#[proc_macro_derive(JsonDecodable, attributes(opcua))]
/// Derive the `JsonDecodable` trait on this struct, creating code
/// to read the struct from an OPC-UA stream with reversible encoding.
///
/// All fields must be marked with `opcua(ignore)` or implement `JsonDecodable`.
pub fn derive_json_decodable(item: TokenStream) -> TokenStream {
    match generate_encoding_impl(parse_macro_input!(item), EncodingToImpl::JsonDecode) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(BinaryEncodable, attributes(opcua))]
/// Derive the `BinaryEncodable` trait on this struct, creating code
/// to write the struct to an OPC-UA binary stream.
///
/// All fields must be marked with `opcua(ignore)` or implement `BinaryEncodable`.
pub fn derive_binary_encodable(item: TokenStream) -> TokenStream {
    match generate_encoding_impl(parse_macro_input!(item), EncodingToImpl::BinaryEncode) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(BinaryDecodable, attributes(opcua))]
/// Derive the `BinaryDecodable` trait on this struct, creating code
/// to read the struct from an OPC-UA binary stream.
///
/// All fields must be marked with `opcua(ignore)` or implement `BinaryDecodable`.
pub fn derive_binary_decodable(item: TokenStream) -> TokenStream {
    match generate_encoding_impl(parse_macro_input!(item), EncodingToImpl::BinaryDecode) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
