

extern crate proc_macro;
extern crate syn;

use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream as TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, DeriveInput, parse_macro_input, Data, DataEnum, Type, Attribute, AttributeArgs, NestedMeta, Meta, Lit, MetaNameValue, PatTupleStruct, Expr};
use syn::parse::{Parse, ParseStream};
use std::iter::Peekable;
use syn::token::Paren;
use std::iter::once;
use syn::parse::ParseBuffer;

#[proc_macro_derive(NodeType)]
pub fn node_type_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    impl_node_type_macro(&name)
}

fn impl_node_type_macro(name: &Ident) -> TokenStream {
    let gen = quote! {
        impl NodeType for #name {
            fn get_type(&self) -> String {
                String::from(stringify!(#name))
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn node(args: TokenStream, body: TokenStream) -> TokenStream {
    unimplemented!()
}

fn impl_node_macro(node_start: &PatTupleStruct) -> TokenStream {
    unimplemented!()
}


#[proc_macro_derive(NodeEnum)]
pub fn node_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    match ast.data {
        Data::Enum(data) => impl_node_enum_macro(&name, &data),
        _ => panic!("NodeEnum can only be dervied on enums.")
    }
}

#[derive(Clone)]
struct EnumVariant
where Self: Sized {
    name: Ident,
    node: Type,
}

fn impl_node_enum_macro(name: &Ident, data: &DataEnum) -> TokenStream {
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

    let variants_parse = enum_variants
        .clone()
        .map(|variant| {
            let node = &variant.node;
            let name = &variant.name;
            quote! {
                let result = #node::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::#name(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
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
        .collect::<Vec<_>>();;

    let my_enum = name;

    let gen = quote! {
        impl<'source> Parse<'source, Token> for #my_enum {
            fn parse(input: &mut ParseBuffer) -> crate::token::Result<'source, Self> {
                #(#variants_parse)*
                Err(crate::parser::ParseFailure::EnumCheck)
            }

            fn span(&self) -> &logos::Span {
                match *self {
                    #(#variants_span)*
                }
            }
        }
    };

    gen.into()
}