use syn::{parse::Parse, Ident, LitStr, Token};

use crate::utils::ItemAttr;

#[derive(Debug, Default)]
pub(crate) struct EncodingFieldAttribute {
    pub rename: Option<String>,
    pub ignore: bool,
    pub required: bool,
}

impl Parse for EncodingFieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut slf = Self::default();

        loop {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "rename" => {
                    input.parse::<Token![=]>()?;
                    let val: LitStr = input.parse()?;
                    slf.rename = Some(val.value());
                }
                "ignore" => {
                    slf.ignore = true;
                }
                "required" => {
                    slf.required = true;
                }
                _ => return Err(syn::Error::new_spanned(ident, "Unknown attribute value")),
            }
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(slf)
    }
}

impl ItemAttr for EncodingFieldAttribute {
    fn combine(&mut self, other: Self) {
        self.rename = other.rename;
        self.ignore |= other.ignore;
        self.required |= other.required;
    }
}
