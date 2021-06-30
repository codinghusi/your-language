

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, DeriveInput, parse_macro_input, Data, DataEnum, Type };
use syn::__private::TokenStream2;

#[proc_macro_derive(NodeType)]
pub fn node_type_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    impl_node_macro(&name)
}

fn impl_node_macro(name: &Ident) -> TokenStream {
    let gen = quote! {
        impl NodeType for #name {
            fn get_type(&self) -> String {
                String::from(stringify!(#name))
            }
        }
    };
    gen.into()
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
            let result = #node::parse(lexer);
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
            fn parse_any(lexer: &mut Lexer<Token>) -> Option<Self> {
                #(#variants)*
                None
            }
        }
    };

    gen.into()
}