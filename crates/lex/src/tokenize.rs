use crate::{pest_parse::Rule, token::Token};

pub struct TokenIterator {
    index: usize,
    tokens: Vec<Token>,
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            self.index += 1;
            Some(self.tokens[self.index - 1].clone())
        } else {
            None
        }
    }
}

pub trait IntoTokenIterator {
    fn into_token_iter(self) -> TokenIterator;
}

impl IntoTokenIterator for pest::iterators::Pair<'_, Rule> {
    fn into_token_iter(self) -> TokenIterator {
        let mut tokens: Vec<_> = self
            .into_inner()
            // ### hack to skip the last iterator
            .rev()
            .skip(1)
            .rev()
            // ### hack ends
            .map(|pr| {
                //
                let str_repr = pr.as_str();
                match pr.as_rule() {
                    // literals
                    Rule::int => Token::Int(str_repr.parse().unwrap()),
                    Rule::float => Token::Float(str_repr.parse().unwrap()),
                    Rule::string => Token::String(str_repr.to_string()),
                    Rule::bool => Token::Bool(str_repr.parse().unwrap()),
                    Rule::teer => Token::Teer(str_repr.parse().unwrap()),
                    Rule::nil => Token::Nil,

                    // signs
                    Rule::neq => Token::Neq,
                    Rule::and => Token::And,
                    Rule::leq => Token::Leq,
                    Rule::eq => Token::Eq,
                    Rule::geq => Token::Geq,
                    Rule::or => Token::Or,
                    Rule::not => Token::Not,
                    Rule::hash => Token::Hash,
                    Rule::percent => Token::Percent,
                    Rule::lpar => Token::Lpar,
                    Rule::rpar => Token::Rpar,
                    Rule::star => Token::Star,
                    Rule::plus => Token::Plus,
                    Rule::comma => Token::Comma,
                    Rule::minus => Token::Minus,
                    Rule::dot => Token::Dot,
                    Rule::slash => Token::Slash,
                    Rule::colon => Token::Colon,
                    Rule::semicolon => Token::Semicolon,
                    Rule::lt => Token::Lt,
                    Rule::assign => Token::Assign,
                    Rule::gt => Token::Gt,
                    Rule::matmul => Token::Matmul,
                    Rule::lbracket => Token::Lbracket,
                    Rule::rbracket => Token::Rbracket,
                    Rule::lbrace => Token::Lbrace,
                    Rule::rbrace => Token::Rbrace,

                    // reserved
                    Rule::r_let => Token::Let,
                    Rule::r_func => Token::Func,
                    Rule::r_typeof => Token::Typeof,
                    Rule::r_if => Token::If,
                    Rule::r_else => Token::Else,
                    Rule::r_model => Token::Model,
                    Rule::r_print => Token::Print,
                    Rule::r_for => Token::For,
                    Rule::r_while => Token::While,
                    Rule::r_return => Token::Return,
                    Rule::r_new => Token::New,

                    // identifier
                    Rule::ident => Token::Identifier(str_repr.to_string()),

                    other => {
                        eprintln!("Unexpected rule: {:?}", other);
                        unreachable!()
                    }
                }
            })
            .collect();

        tokens.push(Token::Eof);

        TokenIterator { index: 0, tokens }
    }
}
