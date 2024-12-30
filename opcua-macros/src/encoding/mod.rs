use attribute::EncodingFieldAttribute;
use binary::{generate_binary_decode_impl, generate_binary_encode_impl};
use json::{generate_json_decode_impl, generate_json_encode_impl};
use proc_macro2::TokenStream;
use syn::DeriveInput;
use xml::generate_xml_impl;

use crate::utils::{EmptyAttribute, StructItem};

mod attribute;
mod binary;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "xml")]
mod xml;

pub(crate) type EncodingStruct = StructItem<EncodingFieldAttribute, EmptyAttribute>;

pub(crate) fn parse_encoding_input(input: DeriveInput) -> syn::Result<EncodingStruct> {
    EncodingStruct::from_input(input)
}

pub enum EncodingToImpl {
    BinaryEncode,
    BinaryDecode,
    #[cfg(feature = "json")]
    JsonEncode,
    #[cfg(feature = "json")]
    JsonDecode,
    #[cfg(feature = "xml")]
    FromXml,
}

pub fn generate_encoding_impl(
    input: DeriveInput,
    target: EncodingToImpl,
) -> syn::Result<TokenStream> {
    let input = parse_encoding_input(input)?;

    match target {
        EncodingToImpl::BinaryEncode => generate_binary_encode_impl(input),
        EncodingToImpl::BinaryDecode => generate_binary_decode_impl(input),
        #[cfg(feature = "json")]
        EncodingToImpl::JsonEncode => generate_json_encode_impl(input),
        #[cfg(feature = "json")]
        EncodingToImpl::JsonDecode => generate_json_decode_impl(input),
        #[cfg(feature = "xml")]
        EncodingToImpl::FromXml => generate_xml_impl(input),
    }
}
