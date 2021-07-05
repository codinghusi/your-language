#![feature(prelude_import)]
#![allow(dead_code)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
#[macro_use]
mod token {
    use logos::{Logos, Lexer, SpannedIter, Span};
    use crate::nodes::eater::separator::{SeparationEater, SeparatedEater};
    use std::iter::Peekable;
    use std::fmt;
    use crate::parser;
    pub enum Brace {
        Open,
        Close,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Brace {
        #[inline]
        fn clone(&self) -> Brace {
            match (&*self,) {
                (&Brace::Open,) => Brace::Open,
                (&Brace::Close,) => Brace::Close,
            }
        }
    }
    pub enum Token {
        # [regex ("[a-zA-Z_][a-zA-Z0-9_]*" , | lex | lex . slice () . to_string ())]
        Identifier(String),
        # [regex ("[{}]" , | lex | if lex . slice () . eq ("{") { Brace :: Open } else { Brace :: Close })]
        CurlyBrace(Brace),
        # [regex ("[()]" , | lex | if lex . slice () . eq ("(") { Brace :: Open } else { Brace :: Close })]
        RoundedBrace(Brace),
        #[token("=>")]
        Assign,
        # [regex (r"/([^\\/]|\\.)+/" , | lex | lex . slice () . to_string ())]
        Regex(String),
        # [regex (r"/[-~][!>]?>/" , | lex | SeparationEater :: fromRaw (lex . slice ()))]
        Separator(SeparationEater),
        # [regex ("/\"([^\"])+\"/" , | lex | lex . slice () . to_string ())]
        String(String),
        # [regex ("([a-zA-Z_][a-zA-Z0-9_]*):" , | lex | lex . slice () . to_string ())]
        EaterName(String),
        #[token(";")]
        Semicolon,
        #[regex(r"[ \t\n]+", logos::skip)]
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
            fn goto1_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::Identifier, lex);
            }
            #[inline]
            fn pattern0(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 1 > 0
            }
            #[inline]
            fn goto27_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::EaterName, lex);
            }
            #[inline]
            fn goto52_ctx1_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto1_ctx1_x(lex),
                };
                match byte {
                    byte if pattern0(byte) => {
                        lex.bump_unchecked(1usize);
                        goto52_ctx1_x(lex)
                    }
                    b':' => {
                        lex.bump_unchecked(1usize);
                        goto27_ctx1_x(lex)
                    }
                    _ => goto1_ctx1_x(lex),
                }
            }
            #[inline]
            fn goto31_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Semicolon);
            }
            #[inline]
            fn goto6_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, Brace, Token> {
                    if lex.slice().eq("(") {
                        Brace::Open
                    } else {
                        Brace::Close
                    }
                }
                callback(lex).construct(Token::RoundedBrace, lex);
            }
            #[inline]
            fn goto32_ctx32_x<'s>(lex: &mut Lexer<'s>) {
                logos::skip(lex).construct(|()| Token::Error, lex);
            }
            #[inline]
            fn pattern1(byte: u8) -> bool {
                match byte {
                    9u8..=10u8 | 32u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto33_ctx32_x<'s>(lex: &mut Lexer<'s>) {
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
                                                                                return goto32_ctx32_x (lex) ;
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto32_ctx32_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto32_ctx32_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto32_ctx32_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto32_ctx32_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto32_ctx32_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto32_ctx32_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto32_ctx32_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto32_ctx32_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto32_ctx32_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto32_ctx32_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto32_ctx32_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto32_ctx32_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto32_ctx32_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto32_ctx32_x(lex);
                    }
                    return goto32_ctx32_x(lex);
                }
                while lex.test(pattern1) {
                    lex.bump_unchecked(1);
                }
                goto32_ctx32_x(lex);
            }
            #[inline]
            fn goto8_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Token::Assign);
            }
            #[inline]
            fn goto54_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some(b">") => {
                        lex.bump_unchecked(2usize);
                        goto8_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto9_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::Regex, lex);
            }
            #[inline]
            fn goto10_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"/") => {
                        lex.bump_unchecked(1usize);
                        goto9_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto9_ctx10_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::Regex, lex);
            }
            #[inline]
            fn goto10_ctx10_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b"/") => {
                        lex.bump_unchecked(1usize);
                        goto9_ctx10_x(lex)
                    }
                    _ => goto10_x(lex),
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
            fn goto13_at1_ctx10_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto10_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(2usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => goto10_x(lex),
                }
            }
            #[inline]
            fn pattern3(byte: u8) -> bool {
                COMPACT_TABLE_0[byte as usize] & 2 > 0
            }
            #[inline]
            fn goto11_ctx10_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto10_ctx10_x(lex),
                };
                match byte {
                    b'\\' => goto13_at1_ctx10_x(lex),
                    byte if pattern3(byte) => {
                        lex.bump_unchecked(1usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => goto10_ctx10_x(lex),
                }
            }
            #[inline]
            fn goto13_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(3usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto13_at2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return lex.error(),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(3usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto22_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::String, lex);
            }
            #[inline]
            fn goto49_at1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J13,
                    J11,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J22, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return lex.error(),
                };
                match LUT[byte as usize] {
                    Jump::J13 => goto13_at2_x(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(2usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::J22 => {
                        lex.bump_unchecked(2usize);
                        goto22_x(lex)
                    }
                    Jump::__ => lex.error(),
                }
            }
            #[inline]
            fn goto23_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"\"/") => {
                        lex.bump_unchecked(2usize);
                        goto22_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto22_ctx23_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::String, lex);
            }
            #[inline]
            fn goto23_ctx23_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"\"/") => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx23_x(lex)
                    }
                    _ => goto23_x(lex),
                }
            }
            #[inline]
            fn pattern4(byte: u8) -> bool {
                match byte {
                    0u8..=b'!' | b'#'..=255u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto24_ctx23_x<'s>(lex: &mut Lexer<'s>) {
                while let Some(arr) = lex.read::<&[u8; 16]>() {
                    if pattern4(arr[0]) {
                        if pattern4(arr[1]) {
                            if pattern4(arr[2]) {
                                if pattern4(arr[3]) {
                                    if pattern4(arr[4]) {
                                        if pattern4(arr[5]) {
                                            if pattern4(arr[6]) {
                                                if pattern4(arr[7]) {
                                                    if pattern4(arr[8]) {
                                                        if pattern4(arr[9]) {
                                                            if pattern4(arr[10]) {
                                                                if pattern4(arr[11]) {
                                                                    if pattern4(arr[12]) {
                                                                        if pattern4(arr[13]) {
                                                                            if pattern4(arr[14]) {
                                                                                if pattern4(arr[15])
                                                                                {
                                                                                    lex . bump_unchecked (16) ;
                                                                                    continue;
                                                                                }
                                                                                lex.bump_unchecked(
                                                                                    15,
                                                                                );
                                                                                return goto23_ctx23_x (lex) ;
                                                                            }
                                                                            lex.bump_unchecked(14);
                                                                            return goto23_ctx23_x(
                                                                                lex,
                                                                            );
                                                                        }
                                                                        lex.bump_unchecked(13);
                                                                        return goto23_ctx23_x(lex);
                                                                    }
                                                                    lex.bump_unchecked(12);
                                                                    return goto23_ctx23_x(lex);
                                                                }
                                                                lex.bump_unchecked(11);
                                                                return goto23_ctx23_x(lex);
                                                            }
                                                            lex.bump_unchecked(10);
                                                            return goto23_ctx23_x(lex);
                                                        }
                                                        lex.bump_unchecked(9);
                                                        return goto23_ctx23_x(lex);
                                                    }
                                                    lex.bump_unchecked(8);
                                                    return goto23_ctx23_x(lex);
                                                }
                                                lex.bump_unchecked(7);
                                                return goto23_ctx23_x(lex);
                                            }
                                            lex.bump_unchecked(6);
                                            return goto23_ctx23_x(lex);
                                        }
                                        lex.bump_unchecked(5);
                                        return goto23_ctx23_x(lex);
                                    }
                                    lex.bump_unchecked(4);
                                    return goto23_ctx23_x(lex);
                                }
                                lex.bump_unchecked(3);
                                return goto23_ctx23_x(lex);
                            }
                            lex.bump_unchecked(2);
                            return goto23_ctx23_x(lex);
                        }
                        lex.bump_unchecked(1);
                        return goto23_ctx23_x(lex);
                    }
                    return goto23_ctx23_x(lex);
                }
                while lex.test(pattern4) {
                    lex.bump_unchecked(1);
                }
                goto23_ctx23_x(lex);
            }
            #[inline]
            fn goto13_at3_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return lex.error(),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(4usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto49_at2_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J13,
                    J11,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J22, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return lex.error(),
                };
                match LUT[byte as usize] {
                    Jump::J13 => goto13_at3_x(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(3usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::J22 => {
                        lex.bump_unchecked(3usize);
                        goto22_x(lex)
                    }
                    Jump::__ => lex.error(),
                }
            }
            #[inline]
            fn goto48_at1_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J24,
                    J49,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J24, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J49, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return lex.error(),
                };
                match LUT[byte as usize] {
                    Jump::J45 => {
                        lex.bump_unchecked(2usize);
                        goto45_x(lex)
                    }
                    Jump::J24 => {
                        lex.bump_unchecked(2usize);
                        goto24_ctx23_x(lex)
                    }
                    Jump::J49 => goto49_at2_x(lex),
                    Jump::__ => lex.error(),
                }
            }
            #[inline]
            fn goto9_ctx9_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::Regex, lex);
            }
            #[inline]
            fn goto22_ctx9_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, String, Token> {
                    lex.slice().to_string()
                }
                callback(lex).construct(Token::String, lex);
            }
            #[inline]
            fn goto47_at1_ctx9_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some(b"/") => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx9_x(lex)
                    }
                    _ => goto9_x(lex),
                }
            }
            #[inline]
            fn goto50_ctx9_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto9_ctx9_x(lex),
                };
                match byte {
                    byte if pattern4(byte) => {
                        lex.bump_unchecked(1usize);
                        goto24_ctx23_x(lex)
                    }
                    b'"' => goto47_at1_ctx9_x(lex),
                    _ => goto9_ctx9_x(lex),
                }
            }
            #[inline]
            fn goto45_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J49,
                    J48,
                    J50,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J49, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J50, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J48, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return lex.error(),
                };
                match LUT[byte as usize] {
                    Jump::J45 => {
                        lex.bump_unchecked(1usize);
                        goto45_x(lex)
                    }
                    Jump::J49 => goto49_at1_x(lex),
                    Jump::J48 => goto48_at1_x(lex),
                    Jump::J50 => {
                        lex.bump_unchecked(1usize);
                        goto50_ctx9_x(lex)
                    }
                    Jump::__ => lex.error(),
                }
            }
            #[inline]
            fn goto13_at5<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(5usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(6usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto49_at4<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J13,
                    J11,
                    J22,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J22, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J13 => goto13_at5(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(5usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::J22 => {
                        lex.bump_unchecked(5usize);
                        goto22_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto48_at3<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J24,
                    J49,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J24, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J49, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => {
                        lex.bump_unchecked(4usize);
                        goto45_x(lex)
                    }
                    Jump::J24 => {
                        lex.bump_unchecked(4usize);
                        goto24_ctx23_x(lex)
                    }
                    Jump::J49 => goto49_at4(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto44_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J45,
                    J48,
                    J11,
                    J50,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J11, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J50, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J48, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45, J45,
                        J45,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J45 => {
                        lex.bump_unchecked(3usize);
                        goto45_x(lex)
                    }
                    Jump::J48 => goto48_at3(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(3usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::J50 => {
                        lex.bump_unchecked(3usize);
                        goto50_ctx9_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto13_at3<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(4usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto17_ctx11_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(
                    lex: &mut Lexer<'s>,
                ) -> impl CallbackResult<'s, SeparationEater, Token> {
                    SeparationEater::fromRaw(lex.slice())
                }
                callback(lex).construct(Token::Separator, lex);
            }
            #[inline]
            fn goto13_at2_ctx11_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto11_ctx10_x(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(3usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => goto11_ctx10_x(lex),
                }
            }
            #[inline]
            fn goto40_at1_ctx11_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J17,
                    J13,
                    J11,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J17, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto11_ctx10_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J17 => {
                        lex.bump_unchecked(2usize);
                        goto17_ctx11_x(lex)
                    }
                    Jump::J13 => goto13_at2_ctx11_x(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(2usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::__ => goto11_ctx10_x(lex),
                }
            }
            #[inline]
            fn goto39_ctx11_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 1usize]>() {
                    Some(b">") => goto40_at1_ctx11_x(lex),
                    _ => goto11_ctx10_x(lex),
                }
            }
            #[inline]
            fn goto13_at4<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(5usize);
                        goto11_ctx10_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto17_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(
                    lex: &mut Lexer<'s>,
                ) -> impl CallbackResult<'s, SeparationEater, Token> {
                    SeparationEater::fromRaw(lex.slice())
                }
                callback(lex).construct(Token::Separator, lex);
            }
            #[inline]
            fn goto40_at4<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J17,
                    J13,
                    J11,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J17, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(4usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J17 => {
                        lex.bump_unchecked(5usize);
                        goto17_x(lex)
                    }
                    Jump::J13 => goto13_at5(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(5usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto41_at3<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J13,
                    J17,
                    J40,
                    J11,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J17, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J40, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(3usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J13 => goto13_at4(lex),
                    Jump::J17 => {
                        lex.bump_unchecked(4usize);
                        goto17_x(lex)
                    }
                    Jump::J40 => goto40_at4(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(4usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto36_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J13,
                    J39,
                    J9,
                    J41,
                    J11,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J39, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J9, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J41, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J13 => goto13_at3(lex),
                    Jump::J39 => {
                        lex.bump_unchecked(3usize);
                        goto39_ctx11_x(lex)
                    }
                    Jump::J9 => {
                        lex.bump_unchecked(3usize);
                        goto9_x(lex)
                    }
                    Jump::J41 => goto41_at3(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(3usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto43_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J13,
                    J44,
                    J36,
                    J11,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J44, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J36, J11, __, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J13, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J36, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11, J11,
                        J11,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J13 => goto13_at2(lex),
                    Jump::J44 => goto44_at2(lex),
                    Jump::J36 => goto36_at2(lex),
                    Jump::J11 => {
                        lex.bump_unchecked(2usize);
                        goto11_ctx10_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto4_x<'s>(lex: &mut Lexer<'s>) {
                #[inline]
                fn callback<'s>(lex: &mut Lexer<'s>) -> impl CallbackResult<'s, Brace, Token> {
                    if lex.slice().eq("{") {
                        Brace::Open
                    } else {
                        Brace::Close
                    }
                }
                callback(lex).construct(Token::CurlyBrace, lex);
            }
            #[inline]
            fn goto55<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J52,
                    J31,
                    J6,
                    J33,
                    J54,
                    J43,
                    J4,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __, __, __, __, __, __, __, __, __, J33, J33, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, J33, __, __, __,
                        __, __, __, __, J6, J6, __, __, __, __, __, J43, __, __, __, __, __, __,
                        __, __, __, __, __, J31, __, J54, __, __, __, J52, J52, J52, J52, J52, J52,
                        J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52,
                        J52, J52, J52, J52, J52, __, __, __, __, J52, __, J52, J52, J52, J52, J52,
                        J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52, J52,
                        J52, J52, J52, J52, J52, J52, J4, __, J4, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
                        __, __, __, __, __, __, __, __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return _end(lex),
                };
                match LUT[byte as usize] {
                    Jump::J52 => {
                        lex.bump_unchecked(1usize);
                        goto52_ctx1_x(lex)
                    }
                    Jump::J31 => {
                        lex.bump_unchecked(1usize);
                        goto31_x(lex)
                    }
                    Jump::J6 => {
                        lex.bump_unchecked(1usize);
                        goto6_x(lex)
                    }
                    Jump::J33 => {
                        lex.bump_unchecked(1usize);
                        goto33_ctx32_x(lex)
                    }
                    Jump::J54 => goto54_at1(lex),
                    Jump::J43 => goto43_at1(lex),
                    Jump::J4 => {
                        lex.bump_unchecked(1usize);
                        goto4_x(lex)
                    }
                    Jump::__ => _error(lex),
                }
            }
            goto55(lex)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            match (&*self,) {
                (&Token::Identifier(ref __self_0),) => {
                    Token::Identifier(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::CurlyBrace(ref __self_0),) => {
                    Token::CurlyBrace(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::RoundedBrace(ref __self_0),) => {
                    Token::RoundedBrace(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::Assign,) => Token::Assign,
                (&Token::Regex(ref __self_0),) => {
                    Token::Regex(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::Separator(ref __self_0),) => {
                    Token::Separator(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::String(ref __self_0),) => {
                    Token::String(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::EaterName(ref __self_0),) => {
                    Token::EaterName(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Token::Semicolon,) => Token::Semicolon,
                (&Token::Error,) => Token::Error,
            }
        }
    }
    pub type ParseBuffer<'source> = parser::ParseBuffer<'source, Token>;
    pub type Result<'source, T> = parser::Result<'source, T, Token>;
    pub type ParseToken = parser::ParseToken<Token>;
    pub type ParseError = parser::ParseError<Token>;
    pub type ParseFailure = parser::ParseFailure<Token>;
}
mod node {
    use crate::token::{Token, ParseBuffer};
    use logos::{Lexer, Span};
    use std::iter::Peekable;
    use crate::parser::{Result};
    pub trait NodeType: Sized {
        fn get_type(&self) -> String;
    }
    pub trait NodeEnum<Token>
    where
        Self: Sized,
        Token: Clone,
    {
        fn parse_any<'source>(lexer: &mut ParseBuffer) -> Result<'source, Self, Token>;
    }
}
mod nodes {
    pub mod node_block {
        use logos::{Lexer, Span};
        use crate::token::{Token, Brace, ParseBuffer, Result};
        use crate::nodes::variable_declaration::VariableDeclarationNode;
        use crate::node::{NodeEnum, NodeType};
        use node_derive::{NodeType, NodeEnum};
        use crate::parser::{Parse};
        use crate::nodes::identifier::IdentifierNode;
        pub enum BlockItem {
            VariableDeclaration(VariableDeclarationNode),
        }
        impl<'source> Parse<'source, Token> for BlockItem {
            fn parse(input: &mut ParseBuffer) -> crate::token::Result<'source, Self> {
                let result = VariableDeclarationNode::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::VariableDeclaration(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                Err(crate::parser::ParseFailure::EnumCheck)
            }
            fn span(&self) -> logos::Span {
                match *self {
                    Self::VariableDeclaration(ref value) => value.span(),
                }
            }
        }
        pub struct NodeBlockNode {
            span: Span,
        }
        impl NodeType for NodeBlockNode {
            fn get_type(&self) -> String {
                String::from("NodeBlockNode")
            }
        }
        impl<'source> Parse<'source, Token> for NodeBlockNode {
            fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                {
                    match {
                        let peek = input.peek();
                        if let Some(_) = peek {
                            if match peek {
                                Some(crate::parser::ParseToken {
                                    token:
                                        crate::token::Token::CurlyBrace(crate::token::Brace::Open),
                                    ..
                                }) => true,
                                _ => false,
                            } {
                                if let Some(token) = input.next() {
                                    if let crate::token::Token::CurlyBrace(
                                        crate::token::Brace::Open,
                                    ) = token.token.clone()
                                    {
                                        Ok(((), token))
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                let token = input.peek().unwrap();
                                Err (crate :: token :: ParseFailure :: Poisoned (crate :: token :: ParseError :: Unexpected { expected : < [_] > :: into_vec (box ["crate::token::Token::CurlyBrace(crate::token::Brace::Open)" . to_string ()]) , got : (* token) . clone () , }))
                            }
                        } else {
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::EOF {
                                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                },
                            ))
                        }
                    } {
                        Ok(tuple) => Ok(tuple.1),
                        Err(err) => Err(err),
                    }
                };
                while let Ok(node) = BlockItem::parse(input) {
                    ::core::panicking::panic("not implemented");
                }
                {
                    match {
                        let peek = input.peek();
                        if let Some(_) = peek {
                            if match peek {
                                Some(crate::parser::ParseToken {
                                    token:
                                        crate::token::Token::CurlyBrace(crate::token::Brace::Close),
                                    ..
                                }) => true,
                                _ => false,
                            } {
                                if let Some(token) = input.next() {
                                    if let crate::token::Token::CurlyBrace(
                                        crate::token::Brace::Close,
                                    ) = token.token.clone()
                                    {
                                        Ok(((), token))
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                let token = input.peek().unwrap();
                                Err (crate :: token :: ParseFailure :: Poisoned (crate :: token :: ParseError :: Unexpected { expected : < [_] > :: into_vec (box ["crate::token::Token::CurlyBrace(crate::token::Brace::Close)" . to_string ()]) , got : (* token) . clone () , }))
                            }
                        } else {
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::EOF {
                                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                },
                            ))
                        }
                    } {
                        Ok(tuple) => Ok(tuple.1),
                        Err(err) => Err(err),
                    }
                };
                Ok(Self { span: 0..1 })
            }
            fn span(&self) -> Span {
                self.span.clone()
            }
        }
    }
    pub mod document {
        use crate::node::{NodeEnum, NodeType};
        use logos::{Lexer, Span};
        use crate::token::{Token, ParseBuffer, Result};
        use node_derive::{NodeType, NodeEnum};
        use crate::nodes::node_definition::NodeDefinitionNode;
        use std::borrow::Borrow;
        use crate::parser::Parse;
        pub enum Document {
            Definition(NodeDefinitionNode),
        }
        impl<'source> Parse<'source, Token> for Document {
            fn parse(input: &mut ParseBuffer) -> crate::token::Result<'source, Self> {
                let result = NodeDefinitionNode::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::Definition(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                Err(crate::parser::ParseFailure::EnumCheck)
            }
            fn span(&self) -> logos::Span {
                match *self {
                    Self::Definition(ref value) => value.span(),
                }
            }
        }
        pub struct DocumentNode {
            pub items: Vec<Document>,
            span: Span,
        }
        impl NodeType for DocumentNode {
            fn get_type(&self) -> String {
                String::from("DocumentNode")
            }
        }
        impl<'source> Parse<'source, Token> for DocumentNode {
            fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                let mut items = ::alloc::vec::Vec::new();
                let start = input.span().end;
                let mut end = start;
                while let Ok(item) = Document::parse(input) {
                    items.push(item);
                    end = input.span().end;
                }
                let span = start..end;
                Ok(DocumentNode { items, span })
            }
            fn span(&self) -> Span {
                self.span.clone()
            }
        }
    }
    pub mod identifier {
        use crate::node::{NodeEnum, NodeType};
        use logos::{Lexer, Span};
        use crate::token::{Token, ParseBuffer, Result};
        use node_derive::{NodeType, NodeEnum};
        use crate::parser::Parse;
        pub struct IdentifierNode {
            pub name: String,
            pub span: Span,
        }
        impl NodeType for IdentifierNode {
            fn get_type(&self) -> String {
                String::from("IdentifierNode")
            }
        }
        impl<'source> Parse<'source, Token> for IdentifierNode {
            fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                let (name, token) = {
                    let peek = input.peek();
                    if let Some(_) = peek {
                        if match peek {
                            Some(crate::parser::ParseToken {
                                token: Token::Identifier(name),
                                ..
                            }) => true,
                            _ => false,
                        } {
                            if let Some(token) = input.next() {
                                if let Token::Identifier(name) = token.token.clone() {
                                    Ok((name, token))
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                };
                            }
                        } else {
                            let token = input.peek().unwrap();
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::Unexpected {
                                    expected: <[_]>::into_vec(box [
                                        "Token::Identifier(name)".to_string()
                                    ]),
                                    got: (*token).clone(),
                                },
                            ))
                        }
                    } else {
                        Err(crate::token::ParseFailure::Poisoned(
                            crate::token::ParseError::EOF {
                                expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                            },
                        ))
                    }
                }?;
                Ok(IdentifierNode {
                    name,
                    span: token.span(),
                })
            }
            fn span(&self) -> Span {
                self.span.clone()
            }
        }
    }
    pub mod node_definition {
        use crate::node::{NodeEnum, NodeType};
        use crate::nodes::identifier::IdentifierNode;
        use crate::nodes::node_block::NodeBlockNode;
        use logos::{Span, Lexer};
        use crate::token::{Token, ParseBuffer, Result};
        use node_derive::{NodeType, NodeEnum};
        use std::iter::Peekable;
        #[macro_use]
        use crate::token;
        use crate::parser::Parse;
        pub struct NodeDefinitionNode {
            name: IdentifierNode,
            block: NodeBlockNode,
            span: Span,
        }
        impl NodeType for NodeDefinitionNode {
            fn get_type(&self) -> String {
                String::from("NodeDefinitionNode")
            }
        }
        impl<'source> Parse<'source, Token> for NodeDefinitionNode {
            fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                let span;
                let start = input.peek_span().start.clone();
                let identifier: IdentifierNode = input.parse()?;
                let block: NodeBlockNode = input.parse()?;
                let end = input.peek_span().end.clone();
                span = (start..end);
                Ok(NodeDefinitionNode {
                    name: identifier,
                    block,
                    span,
                })
            }
            fn span(&self) -> Span {
                self.span.clone()
            }
        }
    }
    pub mod variable_declaration {
        use crate::nodes::identifier::IdentifierNode;
        use crate::node::{NodeEnum, NodeType};
        use logos::{Lexer, Span};
        use crate::token::{Token, Brace, ParseBuffer, Result};
        use node_derive::{NodeType, NodeEnum};
        use crate::nodes::eater::string::StringEater;
        use crate::nodes::eater::Eater;
        use crate::parser::Parse;
        pub struct VariableDeclarationNode {
            name: IdentifierNode,
            eater: Eater,
            span: Span,
        }
        impl NodeType for VariableDeclarationNode {
            fn get_type(&self) -> String {
                String::from("VariableDeclarationNode")
            }
        }
        impl<'source> Parse<'source, Token> for VariableDeclarationNode {
            fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                let name: IdentifierNode = input.parse()?;
                {
                    match {
                        let peek = input.peek();
                        if let Some(_) = peek {
                            if match peek {
                                Some(crate::parser::ParseToken {
                                    token:
                                        crate::token::Token::RoundedBrace(crate::token::Brace::Open),
                                    ..
                                }) => true,
                                _ => false,
                            } {
                                if let Some(token) = input.next() {
                                    if let crate::token::Token::RoundedBrace(
                                        crate::token::Brace::Open,
                                    ) = token.token.clone()
                                    {
                                        Ok(((), token))
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                let token = input.peek().unwrap();
                                Err (crate :: token :: ParseFailure :: Poisoned (crate :: token :: ParseError :: Unexpected { expected : < [_] > :: into_vec (box ["crate::token::Token::RoundedBrace(crate::token::Brace::Open)" . to_string ()]) , got : (* token) . clone () , }))
                            }
                        } else {
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::EOF {
                                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                },
                            ))
                        }
                    } {
                        Ok(tuple) => Ok(tuple.1),
                        Err(err) => Err(err),
                    }
                };
                {
                    match {
                        let peek = input.peek();
                        if let Some(_) = peek {
                            if match peek {
                                Some(crate::parser::ParseToken {
                                    token:
                                        crate::token::Token::RoundedBrace(crate::token::Brace::Close),
                                    ..
                                }) => true,
                                _ => false,
                            } {
                                if let Some(token) = input.next() {
                                    if let crate::token::Token::RoundedBrace(
                                        crate::token::Brace::Close,
                                    ) = token.token.clone()
                                    {
                                        Ok(((), token))
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                let token = input.peek().unwrap();
                                Err (crate :: token :: ParseFailure :: Poisoned (crate :: token :: ParseError :: Unexpected { expected : < [_] > :: into_vec (box ["crate::token::Token::RoundedBrace(crate::token::Brace::Close)" . to_string ()]) , got : (* token) . clone () , }))
                            }
                        } else {
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::EOF {
                                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                },
                            ))
                        }
                    } {
                        Ok(tuple) => Ok(tuple.1),
                        Err(err) => Err(err),
                    }
                };
                {
                    match {
                        let peek = input.peek();
                        if let Some(_) = peek {
                            if match peek {
                                Some(crate::parser::ParseToken {
                                    token: Token::Assign,
                                    ..
                                }) => true,
                                _ => false,
                            } {
                                if let Some(token) = input.next() {
                                    if let Token::Assign = token.token.clone() {
                                        Ok(((), token))
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                let token = input.peek().unwrap();
                                Err(crate::token::ParseFailure::Poisoned(
                                    crate::token::ParseError::Unexpected {
                                        expected: <[_]>::into_vec(
                                            box ["Token::Assign".to_string()],
                                        ),
                                        got: (*token).clone(),
                                    },
                                ))
                            }
                        } else {
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::EOF {
                                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                },
                            ))
                        }
                    } {
                        Ok(tuple) => Ok(tuple.1),
                        Err(err) => Err(err),
                    }
                };
                let eater: Eater = input.parse()?;
                let span = name.span().start..input.span().end;
                Ok(Self { name, span, eater })
            }
            fn span(&self) -> Span {
                self.span.clone()
            }
        }
    }
    pub mod eater {
        use logos::{Lexer, Span, Logos};
        use node_derive::{NodeEnum, NodeType};
        use crate::node::{NodeEnum, NodeType};
        use crate::token::{Token, ParseBuffer};
        use crate::nodes::eater::naming::{NamedEater, UnnamedEater};
        use crate::nodes::eater::string::StringEater;
        use crate::nodes::eater::regex::RegexEater;
        use crate::nodes::eater::function::FunctionEater;
        use crate::parser::Parse;
        pub mod naming {
            use crate::nodes::eater::{EaterItem, EaterNode};
            use crate::nodes::identifier::IdentifierNode;
            use crate::nodes::eater::separator::SeparationEater;
            use logos::{Lexer, Span};
            use node_derive::{NodeEnum, NodeType};
            use crate::node::{NodeEnum, NodeType};
            use crate::token::{Token, ParseBuffer, Result};
            use crate::parser::Parse;
            pub struct NamedEater {
                name: IdentifierNode,
                eater: EaterItem,
                span: Span,
            }
            impl NodeType for NamedEater {
                fn get_type(&self) -> String {
                    String::from("NamedEater")
                }
            }
            impl<'source> Parse<'source, Token> for NamedEater {
                fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                    let (name, name_token) = {
                        let peek = input.peek();
                        if let Some(_) = peek {
                            if match peek {
                                Some(crate::parser::ParseToken {
                                    token: Token::EaterName(name),
                                    ..
                                }) => true,
                                _ => false,
                            } {
                                if let Some(token) = input.next() {
                                    if let Token::EaterName(name) = token.token.clone() {
                                        Ok((name, token))
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    };
                                }
                            } else {
                                let token = input.peek().unwrap();
                                Err(crate::token::ParseFailure::Poisoned(
                                    crate::token::ParseError::Unexpected {
                                        expected: <[_]>::into_vec(box [
                                            "Token::EaterName(name)".to_string()
                                        ]),
                                        got: (*token).clone(),
                                    },
                                ))
                            }
                        } else {
                            Err(crate::token::ParseFailure::Poisoned(
                                crate::token::ParseError::EOF {
                                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                },
                            ))
                        }
                    }?;
                    let eater: EaterItem = input.parse()?;
                    Ok(Self {
                        name: IdentifierNode {
                            name: name.clone(),
                            span: name_token.span(),
                        },
                        eater,
                        span: eater.span(),
                    })
                }
                fn span(&self) -> Span {
                    self.span.clone()
                }
            }
            pub struct UnnamedEater {
                eater: EaterItem,
                span: Span,
            }
            impl NodeType for UnnamedEater {
                fn get_type(&self) -> String {
                    String::from("UnnamedEater")
                }
            }
            impl<'source> Parse<'source, Token> for UnnamedEater {
                fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                    let eater: EaterItem = input.parse()?;
                    Ok(Self {
                        eater,
                        span: eater.span(),
                    })
                }
                fn span(&self) -> Span {
                    self.span.clone()
                }
            }
            impl<'source> EaterNode<'source, Token> for NamedEater {}
        }
        pub mod string {
            use crate::node::{NodeEnum, NodeType};
            use logos::{Lexer, Span};
            use crate::token::{Token, ParseBuffer, Result};
            use node_derive::{NodeType, NodeEnum};
            use crate::nodes::eater::EaterNode;
            use crate::parser::Parse;
            pub struct StringEater {
                value: String,
                span: Span,
            }
            impl NodeType for StringEater {
                fn get_type(&self) -> String {
                    String::from("StringEater")
                }
            }
            impl<'source> Parse<'source, Token> for StringEater {
                fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                    let str: String;
                    let token = {
                        match {
                            let peek = input.peek();
                            if let Some(_) = peek {
                                if match peek {
                                    Some(crate::parser::ParseToken {
                                        token: Token::String(str),
                                        ..
                                    }) => true,
                                    _ => false,
                                } {
                                    if let Some(token) = input.next() {
                                        if let Token::String(str) = token.token.clone() {
                                            Ok(((), token))
                                        } else {
                                            {
                                                ::core::panicking::panic(
                                                    "internal error: entered unreachable code",
                                                )
                                            };
                                        }
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    let token = input.peek().unwrap();
                                    Err(crate::token::ParseFailure::Poisoned(
                                        crate::token::ParseError::Unexpected {
                                            expected: <[_]>::into_vec(box [
                                                "Token::String(str)".to_string()
                                            ]),
                                            got: (*token).clone(),
                                        },
                                    ))
                                }
                            } else {
                                Err(crate::token::ParseFailure::Poisoned(
                                    crate::token::ParseError::EOF {
                                        expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                    },
                                ))
                            }
                        } {
                            Ok(tuple) => Ok(tuple.1),
                            Err(err) => Err(err),
                        }
                    }?;
                    Ok(StringEater {
                        value: str.clone(),
                        span: token.span(),
                    })
                }
                fn span(&self) -> Span {
                    self.span.clone()
                }
            }
            impl<'source> EaterNode<'source, Token> for StringEater {}
        }
        pub mod regex {
            use crate::node::{NodeEnum, NodeType};
            use logos::{Lexer, Span};
            use crate::token::{Token, ParseBuffer, Result};
            use node_derive::{NodeType, NodeEnum};
            use crate::nodes::eater::EaterNode;
            use crate::parser::Parse;
            pub struct RegexEater {
                value: String,
                span: Span,
            }
            impl NodeType for RegexEater {
                fn get_type(&self) -> String {
                    String::from("RegexEater")
                }
            }
            impl<'source> Parse<'source, Token> for RegexEater {
                fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                    let str;
                    {
                        match {
                            let peek = input.peek();
                            if let Some(_) = peek {
                                if match peek {
                                    Some(crate::parser::ParseToken {
                                        token: Token::Regex(str),
                                        ..
                                    }) => true,
                                    _ => false,
                                } {
                                    if let Some(token) = input.next() {
                                        if let Token::Regex(str) = token.token.clone() {
                                            Ok(((), token))
                                        } else {
                                            {
                                                ::core::panicking::panic(
                                                    "internal error: entered unreachable code",
                                                )
                                            };
                                        }
                                    } else {
                                        {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                    }
                                } else {
                                    let token = input.peek().unwrap();
                                    Err(crate::token::ParseFailure::Poisoned(
                                        crate::token::ParseError::Unexpected {
                                            expected: <[_]>::into_vec(box [
                                                "Token::Regex(str)".to_string()
                                            ]),
                                            got: (*token).clone(),
                                        },
                                    ))
                                }
                            } else {
                                Err(crate::token::ParseFailure::Poisoned(
                                    crate::token::ParseError::EOF {
                                        expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                                    },
                                ))
                            }
                        } {
                            Ok(tuple) => Ok(tuple.1),
                            Err(err) => Err(err),
                        }
                    };
                    Ok(RegexEater {
                        value: str,
                        span: input.span(),
                    })
                }
                fn span(&self) -> Span {
                    self.span.clone()
                }
            }
            impl<'source> EaterNode<'source, Token> for RegexEater {}
        }
        pub mod function {
            use crate::node::{NodeEnum, NodeType};
            use logos::{Lexer, Span};
            use crate::token::{Token, ParseBuffer, Result};
            use node_derive::{NodeType, NodeEnum, node};
            use crate::nodes::identifier::IdentifierNode;
            use crate::nodes::eater::EaterNode;
            use crate::parser::Parse;
            pub struct FunctionEater {
                name: IdentifierNode,
                span: Span,
            }
            impl NodeType for FunctionEater {
                fn get_type(&self) -> String {
                    String::from("FunctionEater")
                }
            }
            impl<'source> Parse<'source, Token> for FunctionEater {
                fn parse(input: &mut ParseBuffer) -> Result<'source, Self> {
                    ::core::panicking::panic("not implemented")
                }
                fn span(&self) -> Span {
                    self.span.clone()
                }
            }
            impl<'source> EaterNode<'source, Token> for FunctionEater {}
        }
        pub mod separator {
            use crate::nodes::eater::Eater;
            pub enum Whitespace {
                Optional,
                Required,
                NotAllowed,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for Whitespace {
                #[inline]
                fn clone(&self) -> Whitespace {
                    match (&*self,) {
                        (&Whitespace::Optional,) => Whitespace::Optional,
                        (&Whitespace::Required,) => Whitespace::Required,
                        (&Whitespace::NotAllowed,) => Whitespace::NotAllowed,
                    }
                }
            }
            pub struct SeparatedEater {
                separator_before: SeparationEater,
                eater: Eater,
            }
            pub struct SeparationEater {
                whitespace: Whitespace,
                require_following_eater: bool,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for SeparationEater {
                #[inline]
                fn clone(&self) -> SeparationEater {
                    match *self {
                        SeparationEater {
                            whitespace: ref __self_0_0,
                            require_following_eater: ref __self_0_1,
                        } => SeparationEater {
                            whitespace: ::core::clone::Clone::clone(&(*__self_0_0)),
                            require_following_eater: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl SeparationEater {
                pub fn fromRaw(raw: &str) -> SeparationEater {
                    match raw {
                        "->" => Self {
                            whitespace: Whitespace::Optional,
                            require_following_eater: true,
                        },
                        "->>" => Self {
                            whitespace: Whitespace::Required,
                            require_following_eater: true,
                        },
                        "-!>" => Self {
                            whitespace: Whitespace::NotAllowed,
                            require_following_eater: true,
                        },
                        "~>" => Self {
                            whitespace: Whitespace::Optional,
                            require_following_eater: false,
                        },
                        "~>>" => Self {
                            whitespace: Whitespace::Required,
                            require_following_eater: false,
                        },
                        "~!>" => Self {
                            whitespace: Whitespace::NotAllowed,
                            require_following_eater: false,
                        },
                        _ => ::std::rt::begin_panic({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["", " is not a separator eater"],
                                &match (&raw,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ));
                            res
                        }),
                    }
                }
            }
        }
        pub enum Eater {
            Named(NamedEater),
            Unnamed(UnnamedEater),
        }
        impl<'source> Parse<'source, Token> for Eater {
            fn parse(input: &mut ParseBuffer) -> crate::token::Result<'source, Self> {
                let result = NamedEater::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::Named(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                let result = UnnamedEater::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::Unnamed(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                Err(crate::parser::ParseFailure::EnumCheck)
            }
            fn span(&self) -> logos::Span {
                match *self {
                    Self::Named(ref value) => value.span(),
                    Self::Unnamed(ref value) => value.span(),
                }
            }
        }
        pub enum EaterItem {
            String(StringEater),
            Regex(RegexEater),
            Function(FunctionEater),
        }
        impl<'source> Parse<'source, Token> for EaterItem {
            fn parse(input: &mut ParseBuffer) -> crate::token::Result<'source, Self> {
                let result = StringEater::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::String(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                let result = RegexEater::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::Regex(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                let result = FunctionEater::parse(input);
                if let Ok(parsed) = result {
                    return Ok(Self::Function(parsed));
                } else if let Err(err @ crate::parser::ParseFailure::Poisoned(_)) = result {
                    return Err(err);
                }
                Err(crate::parser::ParseFailure::EnumCheck)
            }
            fn span(&self) -> logos::Span {
                match *self {
                    Self::String(ref value) => value.span(),
                    Self::Regex(ref value) => value.span(),
                    Self::Function(ref value) => value.span(),
                }
            }
        }
        pub trait EaterNode<'source, Token>: Parse<'source, Token>
        where
            Token: Logos<'source> + Clone,
        {
        }
    }
}
mod parser {
    use logos::{Span, SpannedIter, Lexer, Logos};
    use std::fmt::{Display, Formatter, Debug};
    use std::iter::Peekable;
    use std::fmt;
    use std::slice::Iter;
    use std::vec::IntoIter;
    use std::marker::PhantomData;
    pub struct ParseToken<Token>
    where
        Self: Sized,
        Token: Clone,
    {
        pub token: Token,
        pub _span: Span,
        pub slice: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Token: ::core::clone::Clone> ::core::clone::Clone for ParseToken<Token>
    where
        Self: Sized,
        Token: Clone,
    {
        #[inline]
        fn clone(&self) -> ParseToken<Token> {
            match *self {
                ParseToken {
                    token: ref __self_0_0,
                    _span: ref __self_0_1,
                    slice: ref __self_0_2,
                } => ParseToken {
                    token: ::core::clone::Clone::clone(&(*__self_0_0)),
                    _span: ::core::clone::Clone::clone(&(*__self_0_1)),
                    slice: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl<Token> ParseToken<Token>
    where
        Token: Clone,
    {
        fn from<'source>(item: (Token, Span), slice: String) -> Self
        where
            Token: Logos<'source>,
        {
            ParseToken {
                token: item.0,
                _span: item.1,
                slice,
            }
        }
        pub fn span(&self) -> Span {
            self._span.clone()
        }
    }
    pub enum ParseError<Token>
    where
        Token: Clone,
    {
        Unexpected {
            expected: Vec<String>,
            got: ParseToken<Token>,
        },
        EOF {
            expected: Vec<String>,
        },
    }
    impl<Token: Clone> fmt::Debug for ParseError<Token> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Unexpected {
                    expected,
                    got: token,
                } => match expected.len() {
                    0 => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["unexpected "],
                        &match (&token.slice,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    )),
                    1 => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["unexpected ", ", expected "],
                        &match (&token.slice, &expected[0]) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    )),
                    _ => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["unexpected ", ", expected one of "],
                        &match (&token.slice, &expected.join(", ")) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                            ],
                        },
                    )),
                },
                Self::EOF { expected } => match expected.len() {
                    0 => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["unexpected end of line"],
                        &match () {
                            () => [],
                        },
                    )),
                    1 => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["unexpected end of line, expected "],
                        &match (&expected[0],) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    )),
                    _ => f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["unexpected end of line, expected one of "],
                        &match (&expected.join(", "),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    )),
                },
            }
        }
    }
    pub enum ParseFailure<Token: Clone> {
        Peeked(ParseError<Token>),
        Poisoned(ParseError<Token>),
        EnumCheck,
    }
    impl<Token: Clone> fmt::Debug for ParseFailure<Token> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Peeked(error) => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Peeking failed: "],
                    &match (&error,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                )),
                Self::Poisoned(error) => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Parsing failed: "],
                    &match (&error,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                )),
                Self::EnumCheck => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Tried to parse <not implemented>, but couldn\'t"],
                    &match () {
                        () => [],
                    },
                )),
            }
        }
    }
    pub type Result<'source, Result, Token: Logos<'source>> =
        std::result::Result<Result, ParseFailure<Token>>;
    pub struct ParseBuffer<'source, Token>
    where
        Token: Logos<'source> + Clone,
    {
        pub lexer: Peekable<IntoIter<ParseToken<Token>>>,
        pub last_span: Option<Span>,
        next_token: Option<(Token, Span)>,
        lifetime_stuff: PhantomData<&'source ()>,
    }
    impl<'source, Token: Clone> ParseBuffer<'source, Token>
    where
        Token: Logos<'source>,
    {
        pub fn from(lexer: &mut Lexer<'source, Token>) -> Self {
            Self {
                lexer: lexer
                    .spanned()
                    .map(|item| ParseToken::from(item, "<not implemented>".to_string()))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .peekable(),
                last_span: None,
                next_token: None,
                lifetime_stuff: PhantomData,
            }
        }
        pub fn parse<G>(&mut self) -> Result<'source, G, Token>
        where
            G: Parse<'source, Token>,
        {
            return G::parse(self);
        }
        pub fn next(&mut self) -> Option<ParseToken<Token>> {
            self.lexer.next()
        }
        pub fn peek(&mut self) -> Option<&ParseToken<Token>> {
            self.lexer.peek()
        }
        pub fn slice(&self) -> String {
            return String::from("<not implemented>");
        }
        pub fn span(&mut self) -> Span {
            if let Some(span) = &self.last_span {
                span.clone()
            } else {
                0..0
            }
        }
        pub fn peek_span(&mut self) -> Span {
            if let Some(item) = self.lexer.peek() {
                item.span()
            } else {
                let end = self.span().end;
                end..end
            }
        }
    }
    pub trait Parse<'source, Token>
    where
        Token: Logos<'source> + Clone,
        Self: Sized,
    {
        fn parse(input: &mut ParseBuffer<'source, Token>) -> Result<'source, Self, Token>;
        fn span(&self) -> Span;
    }
}
use crate::node::{NodeType, NodeEnum};
use crate::token::{Token, ParseBuffer, Result};
use crate::nodes::document::{DocumentNode};
use logos::Logos;
fn main() -> Result<'static, ()> {
    let code = r"
        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }
    ";
    let mut lexer = Token::lexer(code);
    let mut buffer = crate::parser::ParseBuffer::from(&mut lexer);
    let span;
    let start = buffer.peek_span().start.clone();
    let (name, token) = {
        let peek = buffer.peek();
        if let Some(_) = peek {
            if match peek {
                Some(crate::parser::ParseToken {
                    token: Token::Identifier(identifier),
                    ..
                }) => true,
                _ => false,
            } {
                if let Some(token) = buffer.next() {
                    if let Token::Identifier(identifier) = token.token.clone() {
                        Ok((identifier, token))
                    } else {
                        {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        };
                    }
                } else {
                    {
                        ::core::panicking::panic("internal error: entered unreachable code")
                    };
                }
            } else {
                let token = buffer.peek().unwrap();
                Err(crate::token::ParseFailure::Poisoned(
                    crate::token::ParseError::Unexpected {
                        expected: <[_]>::into_vec(
                            box ["Token::Identifier(identifier)".to_string()],
                        ),
                        got: (*token).clone(),
                    },
                ))
            }
        } else {
            Err(crate::token::ParseFailure::Poisoned(
                crate::token::ParseError::EOF {
                    expected: <[_]>::into_vec(box ["$ variant".to_string()]),
                },
            ))
        }
    }?;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["name: ", "\n"],
            &match (&name,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
    let end = buffer.peek_span().end.clone();
    span = (start..end);
    Ok(())
}
