use std::{fs, io::Write};

use colored::Colorize;
use lex::pest_parse::sl_parse_file;
use parse::{display::print_stmt, parser::parser::Parser};

use crate::{
    errors::TwiError,
    runtime::{gc::objects::ObjectInner, runtime::runtime::Runtime},
};

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

    let rt = Runtime::try_new(root);
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
fn test_rt_1() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let tokens = sl_parse_file("../../test_sources/exec_1.sl");
    let root = Parser::new_from_iter(tokens).parse_stmt();
    let root = match root {
        Ok(r) => r,
        Err(e) => {
            eprintln!("\n[Compiler information] Failed to compile: {}\n", e);
            std::process::exit(0);
        }
    };

    let rt = Runtime::try_new(root);
    let mut rt = match rt {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Preprocess error: {:?}", e);
            std::process::exit(0);
        }
    };

    println!("\n{}\n", "[SL info] Program started".green());
    let result = rt.run();
    match result {
        Ok(_) => {
            let msg = format!("\n[SL return]\n>>  Nil\n>>  Program ended.",);
            println!("{}", msg.blue());
            std::process::exit(0);
        }
        Err(TwiError::Return(val)) => {
            let msg = format!(
                "\n[SL return]\n>>  {}\n>>  Program returned.",
                rt.heap.get_value(val)
            );
            println!("{}", msg.blue());
            std::process::exit(0);
        }
        Err(e) => {
            let msg = format!("\n[SL runtime error]\n>>  {}\n>>  Program aborted.", e);
            println!("{}", msg.red());
            std::process::exit(0);
        }
    }
    // dbg!(rt.scopes);
    // dbg!(rt.heap);
}
