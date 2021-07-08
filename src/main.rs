use ast::token::Token;
use ast::lib::parser::{
    buffer::ParseBuffer,
    into::IntoParseBuffer
};
use ast::nodes::document::DocumentNode;
use logos::Logos;

fn main() {

    let code = r#"
        // Comment
        comments {
            comment => "//" -!> until() -> newline();
            comment => "/*" -> until() -> "*/";
        }

        // Whitespace
        whitespace {
            describe() => /\s/s;
        }


        entrypoint {

        }

        node Identifier {
            describe() => value: /[_a-zA-Z]\w*/;
        }

        node String {
            describe() => '"' -!> value: until() -!> '"';
        }
    "#;
    // println!("tokens: {:?}", Token::lexer(code).spanned().map(|(token, span)| token).collect::<Vec<_>>());
    let mut buffer = Token::parse_buffer(code);

    let document: DocumentNode = buffer.parse().unwrap();
    println!("{}", serde_json::to_string(&document).unwrap());
}




