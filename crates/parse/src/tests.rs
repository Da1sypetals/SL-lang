use std::{fs, io::Write};

use lex::pest_parse::sl_parse_file;

use crate::parser::parser::Parser;

#[test]
fn test_base() {
    //
    let tokens = sl_parse_file("../../test_sources/simple_let.sl");
    for tk in tokens {
        // dbg!(tk);
    }

    let tokens = sl_parse_file("../../test_sources/simple_let.sl");
    let root = Parser::new_from_iter(tokens).parse_stmt();

    dbg!(&root.statements);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open("../../result.txt")
        .unwrap();
    file.write_all(format!("{:?}", root.statements).as_bytes())
        .unwrap();
}
