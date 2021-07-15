extern crate proc_macro;
extern crate syn;

use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream as TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Variant, Ident, DeriveInput, parse_macro_input, Data, DataEnum, Type, Attribute, AttributeArgs, NestedMeta, Meta, Lit, LitStr, MetaNameValue, PatTupleStruct, Expr, punctuated::Punctuated, Token, ItemEnum, Fields};
use syn::parse::{Parse, ParseStream};
use std::iter::Peekable;
use syn::token::Paren;
use std::iter::once;
use syn::parse::ParseBuffer;

pub fn impl_token_enum(name: &Ident, data: &ItemEnum) -> TokenStream {
    let variants = &data.variants;
    let variant_values = variants.iter().map(|variant| {
        let mut values = vec![];
        let ident = &variant.ident;
        for attribute in &variant.attrs {
            let name = match attribute.path.get_ident() {
                Some(ident) => ident.to_string(),
                None => continue
            };

            match name.as_str() {
                "values" => {
                    match attribute.parse_meta() {
                        Ok(Meta::List(list)) => {
                            values.append(&mut list.nested.iter().map(|lit| match lit {
                                NestedMeta::Lit(Lit::Str(lit)) => format!("\"{}\"", lit.value()).to_string(),
                                _ => panic!("only string literals are allowed")
                            }).collect());
                        },
                        _ => panic!("please provide your values as strings comma seperated #[values(\"foo\", \"bar\")]")
                    }
                },
                "name" => {
                    match attribute.parse_meta() {
                        Ok(Meta::List(list)) => {
                            match list.nested.iter().next() {
                                Some(NestedMeta::Lit(Lit::Str(lit))) => values.push(lit.value()),
                                _ => panic!("you must pass a string literal #[name(\"foo\")] ")
                            }
                        },
                        _ => panic!("please provide your name a string #[name(\"foo\")]")
                    }
                },
                "token" => {
                    match attribute.parse_meta() {
                        Ok(Meta::List(list)) => {
                            match list.nested.iter().next() {
                                Some(NestedMeta::Lit(Lit::Str(lit))) => values.push(format!("\"{}\"", lit.value()).to_string()),
                                _ => panic!("you must pass a string literal to #[token(..)]")
                            }
                        },
                        _ => panic!("please provide your token a string #[token(\"foo\")]")
                    }
                },
                _ => continue
            }
        }

        let data = match &variant.fields {
            Fields::Named(_) => quote! { {..} }, 
            Fields::Unnamed(_) => quote! { (..) }, 
            Fields::Unit => quote! { }, 
        };

        quote! {
            Self::#ident#data => vec![#(String::from(#values)),*],
        }
    }).collect::<Vec<_>>();
    let gen = quote! {
        impl ErrValues for #name {
            fn err_values(&self) -> Vec<String> {
                match &self {
                    #(#variant_values)*
                    _ => vec![]
                }
            }
        }
    };
    // panic!(gen.to_string());
    gen.into()
}
