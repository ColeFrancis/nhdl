//! # Parser
//!
//! Parses tokens into an AST 
//!
//! ## Invariants
//!
//! - 
//!
//! Author: Cole Francis
//!
//! Last Updated: 06/25/2026

use super::token::{Token, TokenKind};
use super::ast::*;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn peek (&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn parse_prefix(&mut self) -> Expr {
        match self.advance().kind {
            TokenKind::BoolLiteral(n) => Expr::Literal(Literal::Bool(n)),
            TokenKind::IntLiteral(n)  => Expr::Literal(Literal::Int(n)),
            TokenKind::RealLiteral(n) => Expr::Literal(Literal::Real(n)),
            
            TokenKind::Ident(str) => Expr::Ident(str),

            // TokenKind::Minus => {
            //     let rhs = parse_expr_bp();
            //     Expr::Unary(UnaryExpr {op: UnaryOp::Neg, expr: Box::new(rhs)})
            // }
            // TokenKind::BitNot => {
            //     let rhs = parse_expr_bp();
            //     Expr::Unary(UnaryExpr {op: UnaryOp::BitNot, expr: Box::new(rhs)})
            // }

            // TokenKind::LParen => {
            //     let expr = parse_expr_bp(0);
            //     expect(RParen)
            //     expr
            // }
            _ => panic!("unexpected prefix"),
        }
    }

    /*
        parse_expr(min_bp = 0):
            left = parse_prefix();

            while min_bp < binding_power(peek()):
                token = advance()
                left = led(token, left)

            return left
    */
    // fn parse_expr_bp(&mut self, min_bp: u8) -> Expr {
    //     let lhs = self.parse_prefix();

    //     loop {
    //         let op = self.peek().kind;

    //         let (left_bp, right_bp) = 
    //             match self.infix_binding_power(op) {
    //                 Some(bp) => bp,
    //                 None => break,
    //             };

    //             if left_bp < min_bp {
    //                 break;
    //             }

    //             self.advance();
    //     }
    // }

    // fn infix_binding_power(&self, op: TokenKind) -> Option<(u8, u8)> {
    //     match op {
    //         Gt | Ge | Lt | Le => Some((1, 1)),
    //         Plus | Minus      => Some((10, 11)),
    //         Asterisk | Slash  => Some((20, 21)),
    //         Caret             => Some((31, 30)),
    //         _ => None,
    //     }
    // }

    // fn parse_expr_bp(&mut self, min_bp: u8) -> Expr {
    //     let lhs = self.parse_prefix();

    //     loop {
    //         let op = self.peek().kind;

    //         let (left_bp, right_bp) = 
    //             match self.infix_binding_power(op) {
    //                 Some(bp) => bp,
    //                 None => break,
    //             };

    //         if left_bp < min_bp {
    //             break;
    //         }

    //         self.advance();

    //         let rhs = self.parse_expr_bp(right_bp);

    //         lhs = Expr::Binary(BinaryExpr {
    //             left: Box::new(lhs),
    //             op: BinaryOp(op),
    //             right: Box::new(rhs),
    //         });

    //         return lhs;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::token::{TokenKind::*, Span};

    fn build_token_vec(tokens: Vec<TokenKind>) -> Vec<Token> {
        tokens
            .into_iter()
            .map(|x| Token {kind: x, span: Span{line: 0, col: 0}})
            .collect()
    }

    #[test]
    fn test_literal_expr() {
        let kinds: Vec<TokenKind> = vec![IntLiteral(3)];
        let tokens: Vec<Token> = build_token_vec(kinds);

        let mut parser = Parser::new(tokens);

        let result: Expr = parser.parse_prefix();

        assert_eq!(result, Expr::Literal(Literal::Int(3)));
    }

    #[test]
    fn test_ident_expr() {
        let kinds: Vec<TokenKind> = vec![Ident("hey".to_string())];
        let tokens: Vec<Token> = build_token_vec(kinds);

        let mut parser = Parser::new(tokens);

        let result: Expr = parser.parse_prefix();

        assert_eq!(result, Expr::Ident("hey".to_string()));
    }
}