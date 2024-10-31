use lex::{token::Token, tokenize::TokenIterator};

pub struct Parser {
    pub(crate) cur: usize,
    pub(crate) tokens: Vec<Token>,
}

impl Parser {
    pub fn new_from_iter(iter: TokenIterator) -> Self {
        Self {
            cur: 0,
            tokens: iter.collect(),
        }
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { cur: 0, tokens }
    }

    pub fn current(&self) -> Token {
        self.tokens[self.cur].clone()
    }

    pub fn next_nth(&self, n: usize) -> Token {
        if self.cur + n >= self.tokens.len() {
            Token::Eof
        } else {
            self.tokens[self.cur + n].clone()
        }
    }

    /// advance the cur pointer by n
    pub fn advance(&mut self, n: usize) {
        self.cur += n;
    }
}
