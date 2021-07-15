extern crate proc_macro;
extern crate syn;

use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream as TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, DeriveInput, parse_macro_input, Data, DataEnum, Type, Attribute, AttributeArgs, NestedMeta, Meta, Lit, LitStr, MetaNameValue, PatTupleStruct, Expr, punctuated::Punctuated, Token, ItemEnum, Fields};
use syn::parse::{Parse, ParseStream};
use std::iter::Peekable;
use syn::token::Paren;
use std::iter::once;
use syn::parse::ParseBuffer;


#[derive(Clone)]
pub struct EnumVariant
where Self: Sized {
    name: Ident,
    node: Type,
}

pub fn impl_node_enum_macro(name: &Ident, data: &ItemEnum) -> TokenStream {
    let enum_variants = data.variants.iter().map(|variant| {
        let variant_id = variant.ident.clone();
        match variant.fields {
            syn::Fields::Unnamed(ref fields) => {
                let field = fields.unnamed.iter().next().expect("NodeEnums must have a value with Node implementation");
                let field_type = field.ty.clone();
                EnumVariant {
                    name: variant_id,
                    node: field_type
                }
            },
            _ => panic!("NodeEnums must have a value with Node implementation")
        }
    });

    let expected_tokens = Ident::new("expected_tokens", Span::call_site());

    let variants_parse = enum_variants
        .clone()
        .map(|variant| {
            let node = &variant.node;
            let name = &variant.name;
            quote! {
                match #node::parse(input) {
                    Ok(parsed) => return Ok(Self::#name(parsed)),
                    Err(err) => match err {
                        lib::parser::error::ParseError::Poisoned(_) => return Err(err),
                        lib::parser::error::ParseError::Peeked(unexpected) => {
                            let mut expected = unexpected.expected.clone();
                            #expected_tokens.append(&mut expected);
                        },
                        _ => ()
                    }
                };
            }
        })
        .collect::<Vec<_>>();

    let variants_span = enum_variants
        .map(|variant| {
            let node = &variant.node;
            let name = &variant.name;
            quote! {
                Self::#name(ref value) => value.span(),
            }
        })
        .collect::<Vec<_>>();

    let my_enum = name;

    let gen = quote! {
        impl<'source> Parse<'source, crate::token::Token> for #my_enum {
            fn parse(input: &mut lib::parser::buffer::ParseBuffer<'source, crate::token::Token>) -> lib::parser::result::Result<'source, Self, crate::token::Token> {
                use lib::parser::{
                    unexpected::{Unexpected, Got},
                    error::ParseError
                };
                let mut #expected_tokens = vec![];

                #(#variants_parse)*

                // return error, 'cause nothing worked
                match input.peek() {
                    Some(token) => Err(ParseError::Peeked(Unexpected {
                        expected: #expected_tokens,
                        got: Got::Token(token.clone())
                    })),
                    _ => Err(
                        ParseError::Peeked(Unexpected {
                            expected: #expected_tokens,
                            got: Got::EOF
                        })
                    )
                }

            }

            fn span(&self) -> &lib::parser::span::Span {
                match *self {
                    #(#variants_span)*
                }
            }
        }
    };

    gen.into()
}