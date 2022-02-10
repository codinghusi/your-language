// use ast::token::Token;
// use ast::lib::parser::{
//     buffer::ParseBuffer,
//     into::IntoParseBuffer
// };
// use ast::nodes::document::DocumentNode;
// use logos::Logos;

use fsm::machine::Machine;
use fsm::path::Path;

fn main() {
    // let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
    let letters = "abc";
    let _digits = "0123456789";
    // let identifier = Path::new()
    //     .one_of_chars(letters)
    //     .optional(Path::new().cycle(Path::new().one_of_chars(&format!("{}{}", letters, digits))));
    let identifier = Path::new().cycle(Path::new().one_of_chars(letters));
    let whitespace = Path::new().cycle(Path::new().one_of_chars(" \n\t"));
    let optional_whitespace = Path::new().optional(whitespace.clone());

    let parameter = Path::new()
        .capture_text(String::from("name"), identifier.clone())
        .string(":")
        .then(optional_whitespace.clone())
        .capture_text(String::from("type"), identifier.clone());

    let def = Path::new()
        .string("struct")
        .then(whitespace.clone())
        .capture_text(String::from("name"), identifier.clone())
        .then(optional_whitespace.clone())
        .optional(
            Path::new()
                .char('{')
                .optional(whitespace.clone())
                .cycle(
                    Path::new()
                        .capture(String::from("parameters"), parameter)
                        .optional(whitespace.clone())
                        .optional_string(", "), // .then(optional_whitespace.clone()),
                )
                .char('}')
                .then(optional_whitespace.clone()),
        );

    let machine = Machine::from_paths(&vec![Path::new()
        .cycle(Path::new().capture(
            String::from("documents"),
            optional_whitespace.clone().then(def).char(';'),
        ))
        .end()])
    .unwrap();

    /*let parse_result = machine
    .parse_slow(
        r"
    struct Foo {
        param: Type
    };
    struct Bar;",
    )
    .unwrap();*/
    let parse_result = machine.parse_slow(r"struct ; struct cba;").unwrap();
    println!("parse result: {:?}", parse_result);
    let json = machine.result_to_json(&parse_result);
    println!("json: {}", json);
    // println!("{}", machine.export_xstatejs());

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
