use std::fs;

use log::error;
use pest::Parser;
use pest_derive::Parser;

use crate::tokenize::{IntoTokenIterator, TokenIterator};

#[derive(Parser)]
#[grammar = "sl.pest"]
pub struct SlParser;

pub fn sl_parse_file(filename: &str) -> TokenIterator {
    let unparsed_file = fs::read_to_string(filename).expect("cannot read file");
    let tokens = SlParser::parse(Rule::source, &unparsed_file);
    let tokens = match tokens {
        Ok(mut tokens) => tokens.next().unwrap(),
        Err(e) => {
            error!("[compile error] Invalid syntax:\n {}", e);
            std::process::exit(0);
        }
    };

    tokens.into_token_iter()
}
