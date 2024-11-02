use log::error;
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use std::fs;

use crate::tokenize::{IntoTokenIterator, TokenIterator};

#[derive(Parser)]
#[grammar = "sl.pest"]
pub struct SlParser;

pub fn sl_parse_file(filename: &str) -> TokenIterator {
    let unparsed_file = fs::read_to_string(filename).expect("cannot read file");
    // filter comments away
    let source_string = unparsed_file
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n");
    let tokens = SlParser::parse(Rule::source, &source_string);
    let tokens = match tokens {
        Ok(mut tokens) => tokens.next().unwrap(),
        Err(e) => {
            error!("[compile error] Invalid syntax:\n {}", e);
            std::process::exit(0);
        }
    };
    // dbg!(&tokens);

    tokens.into_token_iter()
}
