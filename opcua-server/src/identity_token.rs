// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

use opcua_types::{
    AnonymousIdentityToken, DecodingOptions, ExtensionObject, ObjectId, UAString,
    UserNameIdentityToken, X509IdentityToken,
};

pub(crate) const POLICY_ID_ANONYMOUS: &str = "anonymous";
pub(crate) const POLICY_ID_USER_PASS_NONE: &str = "userpass_none";
pub(crate) const POLICY_ID_USER_PASS_RSA_15: &str = "userpass_rsa_15";
pub(crate) const POLICY_ID_USER_PASS_RSA_OAEP: &str = "userpass_rsa_oaep";
pub(crate) const POLICY_ID_X509: &str = "x509";

/// Identity token representation on the server, decoded from the client.
pub enum IdentityToken {
    None,
    Anonymous(AnonymousIdentityToken),
    UserName(UserNameIdentityToken),
    X509(X509IdentityToken),
    Invalid(ExtensionObject),
}

impl IdentityToken {
    /// Decode an identity token from an extension object received from the client.
    /// Returns `Invalid` if decoding failed.
    pub fn new(o: &ExtensionObject, decoding_options: &DecodingOptions) -> Self {
        if o.is_empty() {
            // Treat as anonymous
            IdentityToken::Anonymous(AnonymousIdentityToken {
                policy_id: UAString::from(POLICY_ID_ANONYMOUS),
            })
        } else if let Ok(object_id) = o.node_id.as_object_id() {
            // Read the token out from the extension object
            match object_id {
                ObjectId::AnonymousIdentityToken_Encoding_DefaultBinary => {
                    if let Ok(token) = o.decode_inner::<AnonymousIdentityToken>(decoding_options) {
                        IdentityToken::Anonymous(token)
                    } else {
                        IdentityToken::Invalid(o.clone())
                    }
                }
                ObjectId::UserNameIdentityToken_Encoding_DefaultBinary => {
                    if let Ok(token) = o.decode_inner::<UserNameIdentityToken>(decoding_options) {
                        IdentityToken::UserName(token)
                    } else {
                        IdentityToken::Invalid(o.clone())
                    }
                }
                ObjectId::X509IdentityToken_Encoding_DefaultBinary => {
                    // X509 certs
                    if let Ok(token) = o.decode_inner::<X509IdentityToken>(decoding_options) {
                        IdentityToken::X509(token)
                    } else {
                        IdentityToken::Invalid(o.clone())
                    }
                }
                _ => IdentityToken::Invalid(o.clone()),
            }
        } else {
            IdentityToken::Invalid(o.clone())
        }
    }
}