// use ast::token::Token;
// use ast::lib::parser::{
//     buffer::ParseBuffer,
//     into::IntoParseBuffer
// };
// use ast::nodes::document::DocumentNode;
// use logos::Logos;

use fsm::machine::machine::Machine;
use fsm::path::Path;
use fsm::{FSM_Builder, FSM};

fn main() {
    let letter = Path::new().one_of_chars("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_");
    let identifier = Path::new().cycle(letter);

    let machine = Machine::from_paths(&vec![Path::new()
        .cycle(
            Path::new().capture(
                String::from("documents"),
                Path::new()
                    .string("struct ")
                    .capture_text(String::from("name"), identifier)
                    .char(';')
                    .optional(Path::new().cycle(Path::new().one_of_chars(" \t\n"))),
            ),
        )
        .end()])
    .unwrap();

    let parse_result = machine.parse_slow("struct Foo; struct Bar;").unwrap();
    println!("parse result: {:?}", parse_result);
    let json = machine.result_to_json(&parse_result);
    println!("json: {}", json);

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
