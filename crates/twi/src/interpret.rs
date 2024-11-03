use clap::Parser;
use twi::{run_program, InterpreterConfig};

fn main() {
    let cfg = InterpreterConfig::parse();
    run_program(cfg);
}
