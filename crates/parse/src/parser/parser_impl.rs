use std::collections::BTreeSet;

use lex::token::Token;

use crate::{
    ast::{
        expr::ExprNode,
        root::Root,
        stmt::{Lvalue, StmtNode},
    },
    errors::{ParserError, ParserResult},
};

use super::{parser::Parser, parser_expr};

pub struct ParserStep {
    stmt: StmtNode,
    step: usize,
}

impl Parser {
    pub fn parse_stmt(&mut self) -> ParserResult<Root> {
        let mut statements: Vec<StmtNode> = Vec::new();

        while self.cur < self.tokens.len() {
            //
            let statement = match self.current() {
                Token::Let => {
                    // parse let stmt
                    self.parse_let()
                }
                Token::Return => {
                    // parse return stmt
                    self.parse_return()
                }
                // function as statement
                Token::Func => self.parse_func(),
                Token::If => match self.parse_if() {
                    Ok(if_step) => {
                        // try to further parse `else`
                        if let Token::Else = self.next_nth(if_step.step) {
                            // if successful
                            match self.try_parse_else(if_step.step) {
                                // combine if step and else step
                                Ok((else_body, else_step)) => Ok(ParserStep {
                                    stmt: StmtNode::IfElse {
                                        cond: if_step.stmt.unwrap_if().0,
                                        if_body: if_step.stmt.unwrap_if().1,
                                        else_body,
                                    },
                                    step: if_step.step + else_step,
                                }),
                                Err(e) => Err(e),
                            }
                        } else {
                            dbg!(self.next_nth(if_step.step + 1));
                            Ok(if_step)
                        }
                    }
                    Err(e) => Err(e),
                },
                Token::Model => self.parse_model(),
                Token::Print => self.parse_print(),
                Token::For => self.parse_for(),
                Token::While => self.parse_while(),
                Token::Lbrace => self.parse_scope(),
                Token::Eof => break,
                Token::Identifier(_) => self.parse_ident(),

                // expression statement
                Token::Int(_) => self.parse_expression_stmt(),
                Token::Float(_) => self.parse_expression_stmt(),
                Token::String(_) => self.parse_expression_stmt(),
                Token::Bool(_) => self.parse_expression_stmt(),
                Token::Teer(_) => self.parse_expression_stmt(),
                Token::Nil => self.parse_expression_stmt(),

                // other
                token => {
                    return Err(ParserError::InvalidSyntax(format!(
                        "Starting token: {:?}, Parse an 'expression statement'",
                        token
                    )));
                }
            };

            match statement {
                Ok(ParserStep { stmt, step }) => {
                    statements.push(stmt);
                    self.advance(step);
                }
                Err(err) => return Err(err),
            }
        }

        Ok(Root { statements })
    }
}

/*

- 使用嵌套if来解析；
- 使用if之外的尾部返回值来表示解析失败情况

*/
// let a.b =
impl Parser {
    // pub fn parse_let(&self) -> ParserResult<ParserStep> {
    //     // match: let ident =
    //     if let Token::Identifier(ident) = self.next_nth(1) {
    //         let mut member_offset = 2;
    //         let mut members = Vec::new();
    //         'parse_members: loop {
    //             match self.next_nth(member_offset) {
    //                 Token::Dot => {
    //                     if let Token::Identifier(member) = self.next_nth(member_offset + 1) {
    //                         members.push(member);
    //                         member_offset += 2;
    //                     } else {
    //                         return Err(ParserError::InvalidSyntax(format!(
    //                             "Expected identfier after member operator `.`"
    //                         )));
    //                     }
    //                 }
    //                 Token::Assign => break 'parse_members,
    //                 token => {
    //                     return Err(ParserError::InvalidSyntax(format!(
    //                         "Invalid member seperator: {:?}",
    //                         token
    //                     )));
    //                 }
    //             }
    //         }
    //         if let Token::Assign = self.next_nth(member_offset) {
    //             // match trailing ;
    //             let mut expr_tokens = Vec::new();
    //             for i in member_offset + 1.. {
    //                 match self.next_nth(i) {
    //                     Token::Eof => {
    //                         return Err(ParserError::UnexpectedEof);
    //                     }
    //                     Token::Semicolon => {
    //                         break;
    //                     }
    //                     other => {
    //                         expr_tokens.push(other);
    //                     }
    //                 }
    //             }
    //             // try parse intermediate tokens into expr
    //             let len = expr_tokens.len();
    //             let target = if members.is_empty() {
    //                 Lvalue::Identifier(ident)
    //             } else {
    //                 Lvalue::Member {
    //                     base: ident,
    //                     members,
    //                 }
    //             };
    //             return Ok(ParserStep {
    //                 stmt: StmtNode::Let {
    //                     target,
    //                     expr: ExprNode::try_from(expr_tokens)?,
    //                 },
    //                 // let <> = <>;
    //                 step: member_offset + len + 2,
    //             });
    //         }
    //     }
    //     Err(crate::errors::ParserError::InvalidSyntax("Let".to_string()))
    // }

    pub fn parse_assign(&self) -> ParserResult<ParserStep> {
        // match: let ident =
        if let Token::Identifier(ident) = self.current() {
            let mut member_offset = 1;
            let mut members = Vec::new();
            'parse_members: loop {
                match self.next_nth(member_offset) {
                    Token::Dot => {
                        if let Token::Identifier(member) = self.next_nth(member_offset + 1) {
                            members.push(member);
                            member_offset += 2;
                        } else {
                            return Err(ParserError::InvalidSyntax(format!(
                                "Expected identfier after member operator `.`"
                            )));
                        }
                    }
                    Token::Assign => break 'parse_members,
                    token => {
                        return Err(ParserError::InvalidSyntax(format!(
                            "Invalid member separator: {:?}",
                            token
                        )));
                    }
                }
            }

            if let Token::Assign = self.next_nth(member_offset) {
                // match trailing ;
                let mut expr_tokens = Vec::new();
                for i in member_offset + 1.. {
                    match self.next_nth(i) {
                        Token::Eof => {
                            return Err(ParserError::UnexpectedEof);
                        }
                        Token::Semicolon => {
                            break;
                        }
                        other => {
                            expr_tokens.push(other);
                        }
                    }
                }
                // try parse intermediate tokens into expr
                let len = expr_tokens.len();
                let target = if members.is_empty() {
                    Lvalue::Identifier(ident)
                } else {
                    Lvalue::Member {
                        base: ident,
                        members,
                    }
                };
                return Ok(ParserStep {
                    stmt: StmtNode::Assign {
                        target,
                        expr: ExprNode::try_from(expr_tokens)?,
                    },
                    // let <> = <>;
                    step: member_offset + len + 2,
                });
            }
        }

        Err(crate::errors::ParserError::InvalidSyntax("Let".to_string()))
    }

    pub fn parse_let(&self) -> ParserResult<ParserStep> {
        // match: let ident =
        if let (Token::Identifier(ident), Token::Assign) = (self.next_nth(1), self.next_nth(2)) {
            // match trailing ;
            let mut expr_tokens = Vec::new();
            for i in 3.. {
                match self.next_nth(i) {
                    Token::Eof => {
                        return Err(ParserError::UnexpectedEof);
                    }
                    Token::Semicolon => {
                        break;
                    }
                    other => {
                        expr_tokens.push(other);
                    }
                }
            }
            // try parse intermediate tokens into expr
            let len = expr_tokens.len();
            return Ok(ParserStep {
                stmt: StmtNode::Let {
                    ident,
                    expr: ExprNode::try_from(expr_tokens)?,
                },
                step: 4 + len,
            });
        }
        Err(crate::errors::ParserError::InvalidSyntax("Let".to_string()))
    }

    pub fn parse_for(&self) -> ParserResult<ParserStep> {
        // match: let ident =
        if let (Token::Identifier(iter), Token::Colon) = (self.next_nth(1), self.next_nth(2)) {
            // parse expr
            {
                let mut n_iter_expr_tokens = Vec::new();
                let mut iexpr = 3;
                // we must use `loop` instead of `for 3..`
                // to break from loop.
                let (body_start, n_iter) = loop {
                    match self.next_nth(iexpr) {
                        // }: for body starts
                        Token::Lbrace => {
                            // { should not come next
                            if iexpr == 3 {
                                return Err(ParserError::InvalidSyntax(
                                    "For needs n_iter!".to_string(),
                                ));
                            } else {
                                break (iexpr + 1, n_iter_expr_tokens.try_into()?);
                            }
                        }
                        Token::Eof => {
                            return Err(ParserError::UnexpectedEof);
                        }
                        token => {
                            n_iter_expr_tokens.push(token);
                        }
                    }
                    iexpr += 1;
                };

                // read until }, get body
                // we must match {} while peeking token forward
                let mut n_lbr = 1;
                let mut body_tokens = Vec::new();
                for i in body_start.. {
                    match self.next_nth(i) {
                        // get EOF before }
                        Token::Eof => {
                            return Err(ParserError::UnexpectedEof);
                        }
                        other => {
                            match other {
                                Token::Lbrace => {
                                    n_lbr += 1;
                                }
                                Token::Rbrace => {
                                    n_lbr -= 1;
                                    if n_lbr == 0 {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                            body_tokens.push(other);
                        }
                    }
                    // dbg!(n_lbr);
                }
                // dbg!(&body_tokens);
                let len = body_tokens.len();
                let body = Parser::new(body_tokens).parse_stmt()?.statements;
                return Ok(ParserStep {
                    stmt: StmtNode::For { iter, n_iter, body },
                    step: body_start + len + 1,
                });
            }
        }
        Err(crate::errors::ParserError::InvalidSyntax("For".to_string()))
    }

    pub fn parse_while(&self) -> ParserResult<ParserStep> {
        let (cond, body, step) = self.parse_single_expr_with_brace()?;
        Ok(ParserStep {
            stmt: StmtNode::While { cond, body },
            step,
        })
    }

    pub fn parse_if(&self) -> ParserResult<ParserStep> {
        let (cond, body, step) = self.parse_single_expr_with_brace()?;
        Ok(ParserStep {
            stmt: StmtNode::If { cond, body },
            step,
        })
    }

    pub fn try_parse_else(&self, start: usize) -> ParserResult<(Vec<StmtNode>, usize)> {
        // read until }, get body
        // we must match {} while peeking token forward
        let mut n_lbr = 1;
        let mut body_tokens = Vec::new();
        for i in start + 2.. {
            match self.next_nth(i) {
                // get EOF before }
                Token::Eof => {
                    return Err(ParserError::UnexpectedEof);
                }
                other => {
                    match other {
                        Token::Lbrace => {
                            n_lbr += 1;
                        }
                        Token::Rbrace => {
                            n_lbr -= 1;
                            if n_lbr == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    body_tokens.push(other);
                }
            }
            // dbg!(n_lbr);
        }
        let len = body_tokens.len();
        let body = Parser::new(body_tokens).parse_stmt()?.statements;
        Ok((body, len + 3))
    }

    pub fn parse_scope(&self) -> ParserResult<ParserStep> {
        // read until }, get body
        // we must match {} while peeking token forward
        let mut n_lbr = 1;
        let mut body_tokens = Vec::new();
        for i in 1.. {
            match self.next_nth(i) {
                // get EOF before }
                Token::Eof => {
                    return Err(ParserError::UnexpectedEof);
                }
                other => {
                    match other {
                        Token::Lbrace => {
                            n_lbr += 1;
                        }
                        Token::Rbrace => {
                            n_lbr -= 1;
                            if n_lbr == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    body_tokens.push(other);
                }
            }
            // dbg!(n_lbr);
        }
        let len = body_tokens.len();
        let body = Parser::new(body_tokens).parse_stmt()?.statements;
        Ok(ParserStep {
            stmt: StmtNode::Scope { body },
            step: len + 2,
        })
    }

    pub fn parse_single_expr_with_brace(&self) -> ParserResult<(ExprNode, Vec<StmtNode>, usize)> {
        // parse expr
        let mut cond_expr_tokens = Vec::new();
        let mut iexpr = 1;
        // we must use `loop` instead of `for 3..`
        // to break from loop.
        let (body_start, cond) = loop {
            match self.next_nth(iexpr) {
                // }: for body starts
                Token::Lbrace => {
                    // { should not come next
                    if iexpr == 1 {
                        return Err(ParserError::InvalidSyntax(
                            "While needs condition!".to_string(),
                        ));
                    } else {
                        break (iexpr + 1, cond_expr_tokens.try_into()?);
                    }
                }
                Token::Eof => {
                    return Err(ParserError::UnexpectedEof);
                }
                token => {
                    cond_expr_tokens.push(token);
                }
            }
            iexpr += 1;
        };

        // read until }, get body
        // we must match {} while peeking token forward
        let mut n_lbr = 1;
        let mut body_tokens = Vec::new();
        for i in body_start.. {
            match self.next_nth(i) {
                // get EOF before }
                Token::Eof => {
                    return Err(ParserError::UnexpectedEof);
                }
                other => {
                    match other {
                        Token::Lbrace => {
                            n_lbr += 1;
                        }
                        Token::Rbrace => {
                            n_lbr -= 1;
                            if n_lbr == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    body_tokens.push(other);
                }
            }
            // dbg!(n_lbr);
        }
        let len = body_tokens.len();
        let body = Parser::new(body_tokens).parse_stmt()?.statements;
        Ok((cond, body, body_start + len + 1))
    }

    pub fn parse_return(&self) -> ParserResult<ParserStep> {
        // match trailing ;
        let mut expr_tokens = Vec::new();
        for i in 1.. {
            match self.next_nth(i) {
                Token::Eof => {
                    return Err(ParserError::InvalidSyntax("Return".to_string()));
                }
                Token::Semicolon => {
                    break;
                }
                other => {
                    expr_tokens.push(other);
                }
            }
        }

        // try parse intermediate tokens into expr
        let len = expr_tokens.len();
        if let Ok(expr) = expr_tokens.try_into() {
            return Ok(ParserStep {
                stmt: StmtNode::Return { expr },
                step: 2 + len,
            });
        }

        Err(crate::errors::ParserError::InvalidSyntax(
            "Return".to_string(),
        ))
    }

    pub fn parse_print(&self) -> ParserResult<ParserStep> {
        // match trailing ;
        let mut expr_tokens = Vec::new();
        for i in 1.. {
            match self.next_nth(i) {
                Token::Eof => {
                    return Err(ParserError::InvalidSyntax("Return".to_string()));
                }
                Token::Semicolon => {
                    break;
                }
                other => {
                    expr_tokens.push(other);
                }
            }
        }

        // try parse intermediate tokens into expr
        let len = expr_tokens.len();
        if let Ok(expr) = expr_tokens.try_into() {
            return Ok(ParserStep {
                stmt: StmtNode::Print { expr },
                step: 2 + len,
            });
        }

        Err(crate::errors::ParserError::InvalidSyntax(
            "Print".to_string(),
        ))
    }

    pub fn parse_func(&self) -> ParserResult<ParserStep> {
        if let (Token::Identifier(name), Token::Lpar) = (self.next_nth(1), self.next_nth(2)) {
            // no param
            let mut ipeek = 3;
            let mut params = Vec::new();
            // parse param list
            loop {
                match self.next_nth(ipeek) {
                    Token::Identifier(param) => {
                        if params.contains(&param) {
                            return Err(ParserError::DuplicateArg(param));
                        } else {
                            params.push(param);
                        }
                    }
                    Token::Rpar => {
                        ipeek += 1;
                        break;
                    }
                    token => {
                        return Err(ParserError::InvalidSyntax(format!(
                            "Expected identifier of rpar, got {:?}",
                            token
                        )))
                    }
                }
                let sep = self.next_nth(ipeek + 1);

                match sep {
                    Token::Comma => {
                        ipeek += 2;
                    }
                    Token::Rpar => {
                        ipeek += 2;
                        break;
                    }
                    sep => {
                        return Err(ParserError::InvalidSyntax(format!(
                            "Expected comma or rpar, got {:?}",
                            sep
                        )))
                    }
                }
            }

            // parse func body
            let body_start = ipeek + 1;
            let mut n_lbr = 1;
            let mut body_tokens = Vec::new();
            for i in body_start.. {
                match self.next_nth(i) {
                    // get EOF before }
                    Token::Eof => {
                        return Err(ParserError::UnexpectedEof);
                    }
                    other => {
                        match other {
                            Token::Lbrace => {
                                n_lbr += 1;
                            }
                            Token::Rbrace => {
                                n_lbr -= 1;
                                if n_lbr == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        body_tokens.push(other);
                    }
                }
                // dbg!(n_lbr);
            }
            let len = body_tokens.len();
            let body = Parser::new(body_tokens).parse_stmt()?.statements;
            return Ok(ParserStep {
                stmt: StmtNode::FuncDef { name, params, body },
                step: body_start + len + 1,
            });
        }
        Err(ParserError::InvalidSyntax("Func def".to_string()))
    }

    fn parse_model(&self) -> ParserResult<ParserStep> {
        if let (Token::Identifier(model_name), Token::Lbrace) = (self.next_nth(1), self.next_nth(2))
        {
            let mut field_start = 3;
            let mut fields = Vec::new();
            loop {
                // try end model
                if let Token::Rbrace = self.next_nth(field_start) {
                    return Ok(ParserStep {
                        stmt: StmtNode::Model {
                            name: model_name,
                            fields,
                        },
                        step: field_start + 1, // + Rbrace
                    });
                }

                // try parse next field
                if let (Token::Identifier(field_name), Token::Comma) =
                    (self.next_nth(field_start), self.next_nth(field_start + 1))
                {
                    //
                    if fields.contains(&field_name) {
                        // duplicate field name
                        return Err(ParserError::DuplicateArg(field_name));
                    }
                    fields.push(field_name);
                    field_start += 2;
                } else {
                    // jump to end: Err
                    break;
                }
            }
        }
        Err(ParserError::InvalidSyntax("Model def".to_string()))
    }

    pub fn parse_expression_stmt(&self) -> ParserResult<ParserStep> {
        // match trailing ;
        let mut expr_tokens = Vec::new();
        for i in 0.. {
            match self.next_nth(i) {
                Token::Eof => {
                    return Err(ParserError::InvalidSyntax("Return".to_string()));
                }
                Token::Semicolon => {
                    break;
                }
                other => {
                    expr_tokens.push(other);
                }
            }
        }

        // try parse intermediate tokens into expr
        let len = expr_tokens.len();
        if let Ok(expr) = expr_tokens.try_into() {
            return Ok(ParserStep {
                stmt: StmtNode::Expression { expr },
                step: 1 + len,
            });
        }

        Err(crate::errors::ParserError::InvalidSyntax(
            "Expression statement".to_string(),
        ))
    }

    fn parse_ident(&self) -> ParserResult<ParserStep> {
        for i in 0.. {
            match self.next_nth(i) {
                Token::Assign => return self.parse_assign(),
                Token::Semicolon => return self.parse_expression_stmt(),
                Token::Eof => return Err(ParserError::UnexpectedEof),
                _ => {
                    // continue
                }
            }
        }

        Err(ParserError::InvalidSyntax(
            "Syntax error: invalid statement".into(),
        ))
    }
}
