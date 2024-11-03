use clap::Parser;
use twi::run_program;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = CliArgs::parse();
    run_program(&args.path);
}
