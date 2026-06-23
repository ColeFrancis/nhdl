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
//! Last Updated: 06/22/2026

use super::token::Token;
//use super::ast::*;

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

    // fn match(&self, kind: Token) -> bool {
    //     self.peek().kind == kind
    // }

    // fn parse_program(&mut self) -> Program {

    // }

    // fn parse_binary_expression(&mut self, min_bp: u8) -> BinaryExpr {
    //     parse_comparison();
    // }

    // fn parse_comparison(&mut self) -> BinaryExpr {

    // }
}