//! # token
//!
//! Contains an enum of all tokens
//!
//! ## Invariants
//!
//! - Keywords are their own variants
//! - Tokens must obey and impelment grammar
//!
//! Author: Cole Francis
//!
//! Last Updated: 06/25/2026

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self {
            kind: kind,
            span: Span {
                line: line,
                col: col,
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Ent,    // net
    Rel,    // rel
    Net,    // net
    Match,  // match
    Sample, // sample

    Input,  // input 
    Output, // output 
    Init,   // init 
    Let,    // let

    Bool,    // Bool
    Impulse, // Impulse
    Int,     // Int
    Real,    // Real
    Mod,     // Mod

    Ident(String),
    BoolLiteral(bool),
    IntLiteral(i64),
    RealLiteral(f64),

    // Punctuation
    Colon,     // :
    Semicolon, // ;
    Comma,     // ,
    Period,    // .

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=

    Arrow,    // ->
    FatArrow, // =>
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Caret,    // ^
    BitNot,  // ~
    Or,       // |

    Unknown(char),
    InvalidNum(String),
    Eof,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}