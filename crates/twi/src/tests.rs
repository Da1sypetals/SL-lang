use colored::Colorize;
use lex::pest_parse::sl_parse_file;
use parse::parser::parser::Parser;

use crate::{run_program, runtime::runtime::runtime::Runtime, InterpreterConfig};

#[test]
fn test_try() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let tokens = sl_parse_file("../../test_sources/exec_0.sl");
    for t in tokens {
        dbg!(t);
    }
}

#[test]
fn test_rt_base() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let tokens = sl_parse_file("../../test_sources/exec_0.sl");
    let root = Parser::new_from_iter(tokens).parse_stmt();
    let root = match root {
        Ok(r) => r,
        Err(e) => {
            eprintln!("\n[Compiler information] Failed to compile: {}\n", e);
            std::process::exit(0);
        }
    };

    let rt = Runtime::try_new(root, 0.8);
    let mut rt = match rt {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Preprocess error: {:?}", e);
            std::process::exit(0);
        }
    };

    println!("\n{}\n", "[SL info] Program started".green());
    let result = rt.run();
    if let Err(e) = result {
        let msg = format!("\n[SL runtime error]\n>>  {}\n>>  Program exited.", e);
        println!("{}", msg.red());
        std::process::exit(0);
    }
    // dbg!(rt.scopes);
    // dbg!(rt.heap);
}

#[test]
fn test_exec_1() {
    run_program(InterpreterConfig::default("../../test_sources/exec_1.sl"));
}

#[test]
fn test_fib() {
    run_program(InterpreterConfig::default("../../test_sources/fib.sl"));
}
