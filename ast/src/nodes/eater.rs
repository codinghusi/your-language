use logos::Logos;
use node_derive::NodeEnum;

use crate::token::Token;
use crate::nodes::eater::naming::{NamedEater, UnnamedEater};
use crate::nodes::eater::string::StringEater;
use crate::nodes::eater::regex::RegexEater;
use crate::nodes::eater::function::FunctionEater;
use lib::parser::parse::Parse;


pub mod naming;
pub mod string;
pub mod regex;
pub mod function;
pub mod separator;
use serde::{Deserialize, Serialize};

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
pub enum Eater {
    Named(NamedEater),
    Unnamed(UnnamedEater)
}

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
pub enum EaterItem {
    String(StringEater),
    Regex(RegexEater),
    Function(FunctionEater)
}

pub trait EaterNode<'source, Token>: Parse<'source, Token>
where Token: Logos<'source> + Clone { }