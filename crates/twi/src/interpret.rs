use clap::Parser;
use twi::{run_program, InterpreterConfig};

fn main() {
    let args = InterpreterConfig::parse();
    run_program(args);
}
