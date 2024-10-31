use std::{fs, io::Write};

use lex::pest_parse::sl_parse_file;
use log::{error, info};

use crate::{display::print_stmt, parser::parser::Parser};

#[test]
fn test_base() {
    //
    // let tokens = sl_parse_file("../../test_sources/simple_let.sl");
    // for tk in tokens {
    //     // dbg!(tk);
    // }
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let tokens = sl_parse_file("../../test_sources/simple_let.sl");
    let root = Parser::new_from_iter(tokens).parse_stmt();
    let root = match root {
        Ok(r) => r,
        Err(e) => {
            eprintln!("\n[Compiler information] Failed to compile: {:?}\n", e);
            std::process::exit(0);
        }
    };

    dbg!(&root.statements);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open("../../result.txt")
        .unwrap();

    for stmt in &root.statements {
        print_stmt(stmt.clone());
    }

    file.write_all(format!("{:?}", root.statements).as_bytes())
        .unwrap();
}
