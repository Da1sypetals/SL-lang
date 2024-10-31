#[macro_use]
use pretty_env_logger;

#[cfg(test)]
pub mod tests;

pub mod statements;
pub mod ast;
pub mod types;
pub mod parser;
pub mod errors;
