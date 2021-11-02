
// use ast::token::Token;
// use ast::lib::parser::{
//     buffer::ParseBuffer,
//     into::IntoParseBuffer
// };
// use ast::nodes::document::DocumentNode;
// use logos::Logos;

use fsm::{FSM, FSM_Builder};
use fsm::{path::Path};
use fsm::machine::machine::Machine;

fn main() {

    let builder = FSM_Builder::from(
        vec![
            Path::new()
                .string("Hello, ")
                .one_of(vec![
                    Path::new().string("World"),
                    Path::new().string("Woorlde"),
                    Path::new().string("Foo"),
                ])
                .cycle(Path::new().string("!"))
                .optional_string(" How are you?")
                .end(0)
        ]
    );

    let machine = builder.build_machine().unwrap();

    // println!("{:?}", machine);
    // println!("{:?}", machine.all_combinations());
    println!("{}", machine.export_xstatejs());

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




