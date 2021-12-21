use lib::parser::parse::Parse;
use logos::Logos;
use node_derive::NodeEnum;
use serde::{Deserialize, Serialize};

use crate::nodes::eater::function::FunctionEater;
use crate::nodes::eater::naming::{NamedEater, UnnamedEater};
use crate::nodes::eater::regex::RegexEater;
use crate::nodes::eater::string::StringEater;
use crate::token::Token;

pub mod function;
pub mod naming;
pub mod regex;
pub mod separator;
pub mod string;

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Eater {
    Named(NamedEater),
    Unnamed(UnnamedEater),
}

#[derive(NodeEnum, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum EaterItem {
    String(StringEater),
    Regex(RegexEater),
    Function(FunctionEater),
}

pub trait EaterNode<'source, Token>: Parse<'source, Token>
where
    Token: Logos<'source> + Clone,
{
}
