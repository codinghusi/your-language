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
    let builder = FSM_Builder::from(vec![Path::new().cycle(
        Path::new().capture(
            String::from("documents"),
            Path::new()
                .string("struct ")
                .cycle(
                    Path::new().capture_text(
                        String::from("name"),
                        Path::new()
                            .one_of_chars("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_"),
                    ),
                )
                .char(';')
                .optional(Path::new().cycle(Path::new().one_of_chars(" \t\n"))),
        ),
    )]);

    // following should parse: "function my_fn() {;;;;;}"
    // and output: { "fn_name": "my_fn" }
    // let builder = FSM_Builder::from(vec![Path::new()
    //     .string("function ")
    //     .cycle(Path::new().capture_text(
    //         String::from("fn_name"),
    //         Path::new().one_of_chars("abcdefghijklmnopqrstuvwxyz_"),
    //     ))
    //     .string("() {")
    //     .cycle(Path::new().char(';'))
    //     .char('}')]);

    // let builder = FSM_Builder::from(
    //     vec![
    //         Path::new()
    //             .string("abc")
    //             .one_of_chars("123")
    //             .optional_string("opt")
    //             .string("789")
    //             .end(0)
    //     ]
    // );

    let machine = builder.build_machine().unwrap();

    // println!("{:?}", machine);
    // println!("{:?}", machine.all_combinations());
    // println!("{}", machine.export_xstatejs());

    // let parse_result = machine.parse_slow("function abc() {;;;;;}").unwrap();
    let parse_result = machine.parse_slow("struct Foo; struct Bar;").unwrap();
    println!("{:?}", parse_result);
    let json = machine.result_to_json(parse_result);
    println!("{}", json);

    // println!("{:?}", parse_result);

    // let builder = FSM_Builder::from(
    //     vec![
    //         Path::new()
    //             // .capture_text(String::from("first_char"), Path::new()
    //             //     .one_of_chars("abc")
    //             // )
    //             .one_of_chars("abc")
    //             .string("123")
    //             .optional_string("lol"),
    //         Path::new()
    //             .string("opt")
    //             .optional_string("ional")
    //     ]
    // );

    // let builder = FSM_Builder::from(
    //     vec![
    //         Path::new()
    //             .one_of_chars("abc")
    //             .string("123"),
    //         Path::new()
    //             .capture_text(String::from("first_char"), Path::new()
    //                 .one_of_chars("123")
    //             )
    //             .string("abc")
    //             .capture_text(String::from("foo"), Path::new()
    //                 .optional_string(String::from("foo"))
    //                 .cycle(Path::new().string("abc"))
    //             ),
    //         Path::new()
    //             .one_of_chars("123")
    //             .string("123"),
    //     ]
    // );

    // let fsm = builder.build();
    // println!("{:?}", fsm.root.borrow().all_combinations());

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
