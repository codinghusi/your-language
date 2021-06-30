#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod token {
    use logos::Logos;
    pub enum Token {
        #[regex(r"[ \t\n]+")]
        Whitespace,
        #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
        Identifier,
        #[regex("[{}]")]
        CurlyBrace,
        #[regex("[()]")]
        RoundedBrace,
        #[token("=>")]
        Assign,
        #[regex(r"/([^\\/]|\\.)+/")]
        Regex,
        #[token(":")]
        Colon,
        #[token(";")]
        Semicolon,
        #[error]
        Error,
    }
    impl<'s> ::logos::Logos<'s> for Token {
        type Extras = ();
        type Source = str;
        const ERROR: Self = Token::Error;
        fn lex(lex: &mut ::logos::Lexer<'s, Self>) {
            use ::logos::internal::{LexerInternal, CallbackResult};
            type Lexer<'s> = ::logos::Lexer<'s, Token>;
            fn _end<'s>(lex: &mut Lexer<'s>) {
                lex.end()
            }
            fn _error<'s>(lex: &mut Lexer<'s>) {
                lex.bump_unchecked(1);
                lex.error();
            }
            static COMPACT_TABLE_0: [u8; 256] = [
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 3, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2,
            ];
            #[inline]
            fn goto21_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Semicolon);
            }
            #[inline]
            fn goto20_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Colon);
            }
            #[inline]
            fn goto4_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Identifier);
            }
            #[inline]
            fn pattern0(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 1 > 0
            }
            #[inline]
            fn goto5_ctx4_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern0(arr[0]) {
                        if pattern0(arr[1]) {
                            if pattern0(arr[2]) {
                                if pattern0(arr[3]) {
                                    if pattern0(arr[4]) {
                                        if pattern0(arr[5]) {
                                            if pattern0(arr[6]) {
                                                if pattern0(arr[7]) {
                                                    if pattern0(arr[8]) {
                                                        if pattern0(arr[9]) {
                                                            if pattern0(arr[10]) {
                                                                if pattern0(arr[11]) {
                                                                    if pattern0(arr[12]) {
                                                                        if pattern0(arr[13]) {
                                                                            if pattern0(arr[14]) {
                                                                                if pattern0(arr[15])
                                                                                {
                                                                                    lex . bump_unchecked (16) ;
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto4_ctx4_x (lex) ;
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto4_ctx4_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto4_ctx4_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto4_ctx4_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto4_ctx4_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto4_ctx4_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto4_ctx4_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto4_ctx4_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto4_ctx4_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto4_ctx4_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto4_ctx4_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto4_ctx4_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto4_ctx4_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto4_ctx4_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto4_ctx4_x(lex);
                    }
                    return goto4_ctx4_x(lex);
                }
                while lex.test(pattern0) {
                    lex.bump_unchecked(1);
                }
                goto4_ctx4_x(lex);
            }
            #[inline]
            fn goto11_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Assign);
            }
            #[inline]
            fn goto22_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some(b">") => {
                        lex.bump_unchecked(2usize);
                        goto11_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto7_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::CurlyBrace);
            }
            #[inline]
            fn goto1_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Whitespace);
            }
            #[inline]
            fn pattern1(byte: u8) -> bool {
                match byte {
                    9u8..=10u8 | 32u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto2_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern1(arr[0]) {
                        if pattern1(arr[1]) {
                            if pattern1(arr[2]) {
                                if pattern1(arr[3]) {
                                    if pattern1(arr[4]) {
                                        if pattern1(arr[5]) {
                                            if pattern1(arr[6]) {
                                                if pattern1(arr[7]) {
                                                    if pattern1(arr[8]) {
                                                        if pattern1(arr[9]) {
                                                            if pattern1(arr[10]) {
                                                                if pattern1(arr[11]) {
                                                                    if pattern1(arr[12]) {
                                                                        if pattern1(arr[13]) {
                                                                            if pattern1(arr[14]) {
                                                                                if pattern1(arr[15])
                                                                                {
                                                                                    lex . bump_unchecked (16) ;
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto1_ctx1_x (lex) ;
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto1_ctx1_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto1_ctx1_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto1_ctx1_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto1_ctx1_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto1_ctx1_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto1_ctx1_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto1_ctx1_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto1_ctx1_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto1_ctx1_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto1_ctx1_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto1_ctx1_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto1_ctx1_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto1_ctx1_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto1_ctx1_x(lex);
                    }
                    return goto1_ctx1_x(lex);
                }
                while lex.test(pattern1) {
                    lex.bump_unchecked(1);
                }
                goto1_ctx1_x(lex);
            }
            #[inline]
            fn goto12_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Regex);
            }
            #[inline]
            fn goto13_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"/") => {
                        lex.bump_unchecked(1usize);
                        goto12_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto12_ctx13_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Regex);
            }
            #[inline]
            fn goto13_ctx13_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"/") => {
                        lex.bump_unchecked(1usize);
                        goto12_ctx13_x(lex)
                    }
                    _ => goto13_x(lex),
                }
            }
            #[inline]
            fn pattern2(byte: u8) -> bool {
                match byte {
                    0u8..=9u8 | 11u8..=255u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto16_at1_ctx13_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto13_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(2usize);
                        goto14_ctx13_x(lex)
                    }
                    _ => goto13_x(lex),
                }
            }
            #[inline]
            fn pattern3(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 2 > 0
            }
            #[inline]
            fn goto14_ctx13_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto13_ctx13_x(lex),
                };
                match byte {
                    b'\\' => goto16_at1_ctx13_x(lex),
                    byte if pattern3(byte) => {
                        lex.bump_unchecked(1usize);
                        goto14_ctx13_x(lex)
                    }
                    _ => goto13_ctx13_x(lex),
                }
            }
            #[inline]
            fn goto16_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(3usize);
                        goto14_ctx13_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto18_at1<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    b'\\' => goto16_at2(lex),
                    byte if pattern3(byte) => {
                        lex.bump_unchecked(2usize);
                        goto14_ctx13_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto9_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::RoundedBrace);
            }
            #[inline]
            fn goto23<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J21,
                    J20,
                    J5,
                    J22,
                    J7,
                    J2,
                    J18,
                    J9,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, J2, J2, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, J2, __, __, __, __, __,
                        __, __, J9, J9, __, __, __, __, __, J18, __, __, __, __, __, __, __, __,
                        __, __, J20, J21, __, J22, __, __, __, J5, J5, J5, J5, J5, J5, J5, J5, J5,
                        J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, __, __,
                        __, __, J5, __, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5,
                        J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J5, J7, __, J7, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return _end(lex),
                };
                match LUT[byte as usize] {
                    Jump::J21 => {
                        lex.bump_unchecked(1usize);
                        goto21_x(lex)
                    }
                    Jump::J20 => {
                        lex.bump_unchecked(1usize);
                        goto20_x(lex)
                    }
                    Jump::J5 => {
                        lex.bump_unchecked(1usize);
                        goto5_ctx4_x(lex)
                    }
                    Jump::J22 => goto22_at1(lex),
                    Jump::J7 => {
                        lex.bump_unchecked(1usize);
                        goto7_x(lex)
                    }
                    Jump::J2 => {
                        lex.bump_unchecked(1usize);
                        goto2_ctx1_x(lex)
                    }
                    Jump::J18 => goto18_at1(lex),
                    Jump::J9 => {
                        lex.bump_unchecked(1usize);
                        goto9_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            goto23(lex)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Token::Whitespace,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Whitespace");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Identifier,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Identifier");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::CurlyBrace,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "CurlyBrace");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::RoundedBrace,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "RoundedBrace");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Assign,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Assign");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Regex,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Regex");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Colon,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Colon");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Semicolon,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Semicolon");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Token::Error,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Error");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Token {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
}
mod document_node {
    use crate::node::{Node, NodeType, parse_one};
    use logos::Lexer;
    use node_derive::{NodeType, NodeEnum};
    use crate::token::Token;
    pub enum Document {
        Import(ImportNode),
        Definition(DefinitionNode),
    }
    pub struct DocumentNode {
        pub items: Vec<Document>,
    }
    impl NodeType for DocumentNode {
        fn get_type(&self) -> String {
            String::from("DocumentNode")
        }
    }
    struct NodeDefinitionNode {}
    impl NodeType for NodeDefinitionNode {
        fn get_type(&self) -> String {
            String::from("NodeDefinitionNode")
        }
    }
    struct ImportNode {}
    struct NodelessDefinitionNode {}
    struct EntrypointDefinitionNode {}
    struct WhitespaceDefinitionNode {}
    struct CommentDefinitionNode {}
    impl Node for NodeDefinitionNode {
        fn parse(lexer: &mut Lexer<Token>) -> Result<Box<Self>, String> {
            Err(String::from("not implemented"))
        }
    }
    pub enum DefinitionNode {
        Node(NodeDefinitionNode),
    }
    impl DefinitionNode {
        fn parse_any(lexer: &mut Lexer<Token>) -> Option<Self> {
            let result = NodeDefinitionNode::parse(lexer);
            if let Ok(parsed) = result {
                return Some(Self::Node(parsed));
            }
            None
        }
    }
}
mod node {
    use crate::token::Token;
    use logos::Lexer;
    type ParserFn<T> = fn(&mut Lexer<Token>) -> Result<T, String>;
    pub trait NodeType {
        fn get_type(&self) -> String;
    }
    pub trait Node: NodeType {
        fn parse(lexer: &mut Lexer<Token>) -> Result<Box<Self>, String>;
    }
    pub fn parse_one(
        lexer: &mut Lexer<Token>,
        nodeParsers: &[&ParserFn<impl Node>],
    ) -> Option<impl Node> {
        for nodeParser in nodeParsers {
            let result = nodeParser(lexer);
            match result {
                Ok(node) => return Some(node),
                Err(error) => continue,
            }
        }
        return None;
    }
}
use crate::node::{NodeType};
use crate::token::Token;
use crate::document_node::DocumentNode;
use logos::Logos;
fn main() {
    let code = r"
        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }
    ";
    let mut lex = Token::lexer(code);
    let node = DocumentNode {
        items: ::alloc::vec::Vec::new(),
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&node.get_type(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
}
