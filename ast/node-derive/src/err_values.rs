extern crate proc_macro;
extern crate syn;

use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream as TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Pat, Ident, DeriveInput, parse_macro_input, Data, DataEnum, Type, Attribute, AttributeArgs, NestedMeta, Meta, Lit, LitStr, MetaNameValue, PatTupleStruct, Expr, punctuated::Punctuated, Token, ItemEnum, Fields};
use syn::parse::{Parse, ParseStream};
use std::iter::Peekable;
use syn::token::Paren;
use std::iter::once;
use syn::parse::ParseBuffer;

#[proc_macro]
pub fn impl_err_values(pat: &Pat) -> TokenStream {
    let path = match pat {
        Pat::Struct(val) => val.path,
        Pat::TupleStruct(val) => val.path,
        Pat::Path(val) => val.path,
        _ => panic!("given pattern variant couldn't be handled")
    };
    let the_enum: Vec<Ident> = path.segments.iter().rev().skip(1).rev().collect();
    let gen = quote! {
        #the_enum.err_values()
    };
    gen.into()
}