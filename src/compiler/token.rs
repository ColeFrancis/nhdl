//! # token
//!
//! Contains an enum of all tokens
//!
//! ## Invariants
//!
//! - Keywords are their own variants
//!
//! Author: Cole Francis
//!
//! Last Updated: 06/13/2026

#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Ent,    // net
    Rel,    // rel
    Net,    // net
    Match,  // match
    Sample, // sample
    Int,    // Int
    Real,   // Real
    Cmp,    // Cmp
    I,      // i (sqrt(-1))
    E,      // e (constant)
    Pi,     // pi (constant)

    Identifier(String),
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

    Arrow,    // ->
    FatArrow, // =>
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Carrot,   // ^

    Unknown(char),
    InvalidNum(String),
}