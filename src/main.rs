use ast::token::Token;
use ast::lib::parser::{
    buffer::ParseBuffer,
    into::IntoParseBuffer
};
use ast::nodes::document::DocumentNode;

fn main() {
    let code = r#"
        node Ident {
            describe() => value: /[_a-zA-Z]\w*/;
        }

        node String {
            describe() => '"' -!> value: until() -!> '"';
        }
    "#;
    // get all tokens: println!("tokens: {:?}", lexer.spanned().map(|(token, span)| token).collect::<Vec<_>>());
    let mut buffer = Token::parse_buffer(code);

    let document: DocumentNode = buffer.parse().unwrap();
    println!("{}", serde_json::to_string(&document).unwrap());
}




