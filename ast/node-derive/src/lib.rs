// #![allow(unused)]
//
// extern crate proc_macro;
// extern crate syn;
//
// use proc_macro2::{TokenStream as TokenStream2, Span};
// use proc_macro::TokenStream as TokenStream;
// use quote::{quote, ToTokens, TokenStreamExt};
// use syn::{
//     Ident, ItemEnum, Variant, parse_macro_input, Pat
// };
// use syn::parse::{Parse, ParseStream};
// use std::iter::Peekable;
// use syn::token::Paren;
// use std::iter::once;
// use syn::parse::ParseBuffer;
//
// mod node_enum;
// mod token_enum;
// mod err_values;
//
// use crate::node_enum::impl_node_enum_macro;
// use crate::token_enum::impl_token_enum;
// use crate::err_values::impl_err_values;
//
// #[proc_macro_derive(NodeEnum)]
// pub fn node_enum_derive(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as ItemEnum);
//     let name = &ast.ident;
//     impl_node_enum_macro(&name, &ast)
// }
//
// #[proc_macro_derive(TokenEnum, attributes(values, name))]
// pub fn token_enum_macro_derive(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as ItemEnum);
//     let name = &ast.ident;
//     impl_token_enum(&name, &ast)
// }
//
// #[proc_macro]
// pub fn err_values(input: TokenStream) -> TokenStream {
//     let pattern = parse_macro_input!(input as Pat);
//     impl_err_values(&pattern)
// }