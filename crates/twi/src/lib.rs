// Rustc please remind me of unused scopeguard
#![forbid(unused_must_use)]

use colored::Colorize;
use errors::TwiError;
use lex::pest_parse::sl_parse_file;
use parse::parser::parser::Parser;
use runtime::runtime::runtime::Runtime;
mod errors;
mod runtime;
mod scope;
#[cfg(test)]
mod tests;

pub fn run_program(path: &str) {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let tokens = sl_parse_file(path);
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
