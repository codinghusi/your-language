

extern crate proc_macro;
#[macro_use]
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

// #[proc_macro]
// pub fn parse(input: TokenStream) -> TokenStream {
//     let result = parse_macro_input!(input as ParseMacro);
//     result.result
// }
//
// struct ParseMacro {
//     result: TokenStream
// }
//
// struct ToTokensTokenStream {
//     token_stream: TokenStream
// }
//
// impl ToTokensTokenStream {
//     fn from(tokens: TokenStream) -> Self {
//         Self { token_stream: tokens }
//     }
// }
//
// impl ToTokens for ToTokensTokenStream {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         let ts = TokenStream2::from(self.token_stream);
//         tokens.append_all(ts);
//     }
// }
//
// impl Parse for ParseMacro {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let lexer: Expr = input.parse()?;
//         input.parse::<Token![,]>()?;
//         let content;
//         braced!(content in input);
//
//         let new = TokenStream2::new();
//         content.step(|token| {
//             if let Ok(parsefn) = content.parse::<ParseFn>() {
//                 let name = parsefn.name;
//                 let args = ToTokensTokenStream::from(parsefn.args);
//                 let replacement = quote! {
//                     #name!(#args)
//                 };
//                 new.append_all(replacement)
//             } else {
//                 new.append(token.token_stream());
//             }
//             Ok(())
//         });
//
//         Ok(Self {
//             result: new
//         })
//     }
// }
//
// struct ParseFn {
//     at: Token![@],
//     name: Ident,
//     args: TokenStream,
//     paren_token: Paren
// }
//
// impl Parse for ParseFn {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let args;
//         Ok(Self {
//             at: input.parse()?,
//             name: input.parse()?,
//             paren_token: parenthesized!(args in input),
//             args: args.parse()?
//         })
//     }
// }

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

struct EnumVariant {
    name: Ident,
    node: Type,
}

impl ToTokens for EnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let node = &self.node;
        let name = &self.name;
        let my_tokens = quote! {
            let result = #node::parse(input);
            if let Ok(parsed) = result {
                return Some(Self::#name(parsed));
            }
        };
        tokens.append_all(my_tokens);
    }
}


fn impl_node_enum_macro(name: &Ident, data: &DataEnum) -> TokenStream {
    let variants: Box<dyn Iterator<Item=EnumVariant>> = Box::new(data.variants.clone().into_iter().map(|variant| {
        let variant_id = variant.ident;
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
    }));

    let my_enum = name;

    let gen = quote! {
        impl NodeEnum for #my_enum {
            fn parse_any(input: &mut ParseBuffer) -> Option<Self> {
                #(#variants)*
                None
            }
        }
    };

    gen.into()
}