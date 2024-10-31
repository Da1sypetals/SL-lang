use std::fs;

use log::info;
use pest::Parser;

use crate::{
    pest_parse::{sl_parse_file, Rule, SlParser},
    tokenize::{IntoTokenIterator, TokenIterator},
};

#[test]
fn test_lex_base() {
    let unparsed_file =
        fs::read_to_string("../../test_sources/test_lex.sl").expect("cannot read file");
    let tokens = SlParser::parse(Rule::source, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    for token in tokens.into_inner() {
        println!("{:?}\n|-------  {}\n", token.as_rule(), token.as_str());
    }
}

#[test]
fn test_lex_iter() {
    pretty_env_logger::init();
    let tokens = sl_parse_file("../../test_sources/test_lex.sl");

    for token in tokens {
        println!("{:?}", token);
    }
}
