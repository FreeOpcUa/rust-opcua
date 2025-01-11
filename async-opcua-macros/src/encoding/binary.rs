use proc_macro2::TokenStream;
use quote::quote;

use super::{enums::SimpleEnum, EncodingStruct};

pub fn generate_binary_encode_impl(strct: EncodingStruct) -> syn::Result<TokenStream> {
    let mut byte_len_body = quote! {};
    let mut encode_body = quote! {};

    for field in strct.fields {
        if field.attr.ignore {
            continue;
        }

        let ident = field.ident;
        byte_len_body.extend(quote! {
            size += self.#ident.byte_len(ctx);
        });
        encode_body.extend(quote! {
            self.#ident.encode(stream, ctx)?;
        });
    }
    let ident = strct.ident;

    Ok(quote! {
        impl opcua::types::BinaryEncodable for #ident {
            #[allow(unused)]
            fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
                let mut size = 0usize;
                #byte_len_body
                size
            }
            #[allow(unused)]
            fn encode<S: std::io::Write + ?Sized>(
                &self,
                stream: &mut S,
                ctx: &opcua::types::Context<'_>,
            ) -> opcua::types::EncodingResult<()> {
                #encode_body
                Ok(())
            }
        }
    })
}

pub fn generate_binary_decode_impl(strct: EncodingStruct) -> syn::Result<TokenStream> {
    let mut decode_impl = quote! {};
    let mut decode_build = quote! {};

    let mut has_context = false;
    for field in strct.fields {
        if field.attr.ignore {
            continue;
        }

        let ident = field.ident;
        let ident_string = ident.to_string();
        if ident_string == "request_header" {
            decode_impl.extend(quote! {
                let request_header: opcua::types::RequestHeader = opcua::types::BinaryDecodable::decode(stream, ctx)?;
                let __request_handle = request_header.request_handle;
            });
            decode_build.extend(quote! {
                request_header,
            });
            has_context = true;
        } else if ident_string == "response_header" {
            decode_impl.extend(quote! {
                let response_header: opcua::types::ResponseHeader = opcua::types::BinaryDecodable::decode(stream, ctx)?;
                let __request_handle = response_header.request_handle;
            });
            decode_build.extend(quote! {
                response_header,
            });
            has_context = true;
        } else if has_context {
            decode_build.extend(quote! {
                #ident: opcua::types::BinaryDecodable::decode(stream, ctx)
                    .map_err(|e| e.with_request_handle(__request_handle))?,
            });
        } else {
            decode_build.extend(quote! {
                #ident: opcua::types::BinaryDecodable::decode(stream, ctx)?,
            });
        }
    }

    let ident = strct.ident;

    Ok(quote! {
        impl opcua::types::BinaryDecodable for #ident {
            #[allow(unused_variables)]
            fn decode<S: std::io::Read + ?Sized>(stream: &mut S, ctx: &opcua::types::Context<'_>) -> opcua::types::EncodingResult<Self> {
                #decode_impl
                Ok(Self {
                    #decode_build
                })
            }
        }
    })
}

pub fn generate_simple_enum_binary_decode_impl(en: SimpleEnum) -> syn::Result<TokenStream> {
    let ident = en.ident;
    let repr = en.repr;

    Ok(quote! {
        impl opcua::types::BinaryDecodable for #ident {
            #[allow(unused_variables)]
            fn decode<S: std::io::Read + ?Sized>(stream: &mut S, ctx: &opcua::types::Context<'_>) -> opcua::types::EncodingResult<Self> {
                let val = #repr::decode(stream, ctx)?;
                Self::try_from(val)
            }
        }
    })
}

pub fn generate_simple_enum_binary_encode_impl(en: SimpleEnum) -> syn::Result<TokenStream> {
    let ident = en.ident;
    let repr = en.repr;

    Ok(quote! {
        impl opcua::types::BinaryEncodable for #ident {
            #[allow(unused)]
            fn byte_len(&self, ctx: &opcua::types::Context<'_>) -> usize {
                (*self as #repr).byte_len(ctx)
            }
            #[allow(unused)]
            fn encode<S: std::io::Write + ?Sized>(
                &self,
                stream: &mut S,
                ctx: &opcua::types::Context<'_>,
            ) -> opcua::types::EncodingResult<()> {
                (*self as #repr).encode(stream, ctx)
            }
        }
    })
}
