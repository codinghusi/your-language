
// use ast::token::Token;
// use ast::lib::parser::{
//     buffer::ParseBuffer,
//     into::IntoParseBuffer
// };
// use ast::nodes::document::DocumentNode;
// use logos::Logos;

use fsm::{FSM, FSM_Builder};
use fsm::EdgeType;
use fsm::{Path, PathItem, Item};

fn foo(str: &str) -> Vec<EdgeType> {
    str.chars().map(|c| EdgeType::Char(c)).collect::<Vec<_>>()
}

fn main() {

    let builder = FSM_Builder::from(
        vec![
            Path::new()
                .oneOf("abc")
                .string("123"),
            Path::new()
                .oneOf("123")
                .string("abc"),
            Path::new()
                .oneOf("123")
                .string("123"),
        ]
    );

    let fsm = builder.build();





    // let fsm = FSM::build(vec![
    //     foo("123"),
    //     foo("abc"),
    //     foo("ab5"),
    // ]);
    //
    // fsm.parse("abc");
    // println!("{:?}", fsm.root.combinations())
    // println!("{:?}", fsm.root.combinations());

    // return;
    // let code = r#"
    //     // // Comment
    //     // comments {
    //     //     comment => "//" -!> until() -> newline();
    //     //     comment => "/*" -> until() -> "*/";
    //     // }
    //
    //     // // Whitespace
    //     // whitespace {
    //     //     describe() => /\s/s;
    //     // }
    //
    //
    //     // entrypoint {
    //
    //     // }
    //
    //
    //     node Identifier {
    //         describe() => -> value: /[_a-zA-Z]\w*/;
    //     }
    //
    //     node String {
    //         describe() => '"' -!> value: until() -!> '"';
    //     }
    // "#;
    // // println!("tokens: {:?}", Token::lexer(code).spanned().map(|(token, span)| token).collect::<Vec<_>>());
    // let mut buffer = Token::parse_buffer(code);
    //
    // let document: DocumentNode = buffer.parse().unwrap();
    // println!("{}", serde_json::to_string(&document).unwrap());
}




