use logos::{Lexer, Span};
use node_derive::{NodeEnum, NodeType};

use crate::node::{Node, NodeEnum, NodeType};
use crate::token::{Token, ParseBuffer};
use crate::nodes::eater::naming::{NamedEater, UnnamedEater};
use crate::nodes::eater::string::StringEater;
use crate::nodes::eater::regex::RegexEater;
use crate::nodes::eater::function::FunctionEater;


pub mod naming;
pub mod string;
pub mod regex;
pub mod function;
pub mod separator;

#[derive(NodeEnum)]
pub enum Eater {
    Named(NamedEater),
    Unnamed(UnnamedEater)
}

#[derive(NodeEnum)]
pub enum EaterItem {
    String(StringEater),
    Regex(RegexEater),
    Function(FunctionEater)
}

pub trait EaterNode: Node { }