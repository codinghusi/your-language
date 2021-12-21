#![allow(unused)]

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::iter::once;
use std::iter::Peekable;
use syn::parse::ParseBuffer;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parse_macro_input, Ident, ItemEnum, Path, Variant};

use crate::node_enum::impl_node_enum_macro;
use crate::token_enum::impl_token_enum;

mod node_enum;
mod token_enum;

#[proc_macro_derive(NodeEnum)]
pub fn node_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let name = &ast.ident;
    impl_node_enum_macro(&name, &ast)
}

#[proc_macro_derive(TokenEnum, attributes(values, name))]
pub fn token_enum_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let name = &ast.ident;
    impl_token_enum(&name, &ast)
}
