use crate::{
    ast::expr::ExprNode,
    errors::{ParserError, ParserResult},
};
use lex::token::Token;

/**
 * 每次成功解析一个token，就必须移动cur，移动到下一个要被解析的token的位置。
 */
pub struct ExprTokens {
    pub(crate) tokens: Vec<Token>,
    pub(crate) cur: usize,
}

impl ExprTokens {
    pub fn is_terminal(&self) -> bool {
        if self.cur < self.tokens.len() {
            false
        } else if self.cur == self.tokens.len() {
            true
        } else {
            panic!("Internal error: current cursor exceeds token length!");
        }
    }

    pub fn current(&self) -> Token {
        self.tokens[self.cur].clone()
    }

    pub fn next_nth(&self, n: usize) -> ParserResult<Token> {
        if self.cur + n >= self.tokens.len() {
            Err(ParserError::UnexpectedEof)
        } else {
            Ok(self.tokens[self.cur + n].clone())
        }
    }
}

/// remember to advance cur poiner
impl ExprTokens {
    pub fn parse_expr(&mut self) -> ParserResult<ExprNode> {
        println!("tokens: {:?}", self.tokens);
        self.parse_equality()
    }

    pub fn parse_equality(&mut self) -> ParserResult<ExprNode> {
        let mut left = self.parse_comparison()?;

        let mut eq_continue = true;
        while eq_continue {
            if self.is_terminal() {
                return Ok(left);
            }
            eq_continue = false;

            if let Token::Eq = self.current() {
                eq_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::eq(left, right);
                continue;
            }

            if let Token::Neq = self.current() {
                eq_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::neq(left, right);
                continue;
            }
        }

        Ok(left)
    }

    pub fn parse_comparison(&mut self) -> ParserResult<ExprNode> {
        let mut left = self.parse_add()?;

        let mut cmp_continue = true;
        while cmp_continue {
            if self.is_terminal() {
                return Ok(left);
            }
            cmp_continue = false;

            if let Token::Gt = self.current() {
                cmp_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::gt(left, right);
                continue;
            }

            if let Token::Lt = self.current() {
                cmp_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::lt(left, right);
                continue;
            }

            if let Token::Geq = self.current() {
                cmp_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::geq(left, right);
                continue;
            }

            if let Token::Leq = self.current() {
                cmp_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::leq(left, right);
                continue;
            }
        }
        Ok(left)
    }

    pub fn parse_add(&mut self) -> ParserResult<ExprNode> {
        let mut left = self.parse_mul()?;

        let mut add_continue = true;
        while add_continue {
            if self.is_terminal() {
                return Ok(left);
            }
            add_continue = false;

            if let Token::Plus = self.current() {
                add_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::add(left, right);
                continue;
            }

            if let Token::Minus = self.current() {
                add_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::minus(left, right);
                continue;
            }
        }
        Ok(left)
    }

    pub fn parse_mul(&mut self) -> ParserResult<ExprNode> {
        let mut left = self.parse_unary()?;

        let mut mul_continue = true;
        while mul_continue {
            if self.is_terminal() {
                return Ok(left);
            }
            mul_continue = false;

            if let Token::Star = self.current() {
                mul_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::mul(left, right);
                continue;
            }

            if let Token::Slash = self.current() {
                mul_continue = true;
                self.cur += 1;
                let right = self.parse_add()?;
                left = ExprNode::div(left, right);
                continue;
            }
        }
        Ok(left)
    }

    pub fn parse_unary(&mut self) -> ParserResult<ExprNode> {
        // UNOP unary_expr
        if let Token::Not = self.current() {
            self.cur += 1;
            return Ok(ExprNode::not(self.parse_unary()?));
        }
        if let Token::Minus = self.current() {
            self.cur += 1;
            return Ok(ExprNode::neg(self.parse_unary()?));
        }

        // ... or atom
        self.parse_atom()
    }

    pub fn parse_atom(&mut self) -> ParserResult<ExprNode> {
        // try to parse new
        if let Token::New = self.current() {
            if let Token::Identifier(typename) = self.next_nth(1)? {
                // offset cur pointer
                self.cur += 2;
                return Ok(ExprNode::New(typename));
            }
        }

        // call preceed identifier
        if let Token::Identifier(ident) = self.current() {
            if self.cur + 1 >= self.tokens.len() {
                self.cur += 1;
                return Ok(ExprNode::Identifer(ident));
            }

            // try to parse chained member
            if let Token::Dot = self.next_nth(1)? {
                let mut member_offset = 1;
                let mut members = Vec::new();
                // iteratively parse member until there is none
                while let Token::Dot = self.next_nth(member_offset)? {
                    if let Token::Identifier(member) = self.next_nth(member_offset + 1)? {
                        println!("{}", member);
                        members.push(member);
                    }
                    member_offset += 2;
                    if self.cur + member_offset >= self.tokens.len() {
                        break;
                    }
                }
                // no more member, or token is eaten up
                // offset cur pointer
                self.cur += member_offset;
                return Ok(ExprNode::Member {
                    base: ident,
                    members,
                });
            }

            // try to parse a call
            if let Token::Lpar = self.next_nth(1)? {
                if let Token::Rpar = self.next_nth(2)? {
                    // call with no arg
                    self.cur += 3;
                    return Ok(ExprNode::Call {
                        name: ident,
                        args: vec![],
                    });
                } else {
                    self.cur += 2;
                    let mut args = Vec::new();
                    'parse_args: loop {
                        let expr = self.parse_expr()?;
                        args.push(expr);
                        let sep = self.current();
                        match sep {
                            Token::Comma => {
                                self.cur += 1;
                            }
                            Token::Rpar => {
                                self.cur += 1;
                                break 'parse_args;
                            }
                            token => {
                                return Err(ParserError::InvalidSyntax(format!(
                                    "Unexpected token in function call: {:?}",
                                    token
                                )))
                            }
                        }
                    }
                    return Ok(ExprNode::call(ident, args));
                }
            } else {
                self.cur += 1;
                return Ok(ExprNode::Identifer(ident));
            }
        }
        if let Ok(lit) = self.current().try_into() {
            self.cur += 1;
            return Ok(ExprNode::Literal(lit));
        }
        if let Token::Lpar = self.current() {
            self.cur += 1;
            let expr = self.parse_expr()?;
            if let Token::Rpar = self.current() {
                self.cur += 1;
                return Ok(ExprNode::packed(expr));
            }
        }

        dbg!(&self.tokens);
        dbg!(&self.cur);

        Err(ParserError::InvalidSyntax(format!(
            "Invalid atomic expression starter: {:?}",
            self.current()
        )))
    }

    pub fn parse_call(&mut self) -> ParserResult<ExprNode> {
        todo!()
    }
}

// member: ident ~ ("." ~ ident)*
