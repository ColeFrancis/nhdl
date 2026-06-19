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
//! Last Updated: 06/19/2026

#[derive(Debug, PartialEq)]
pub enum Token {
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

    Bool,   // Bool
    Int,    // Int
    Real,   // Real
    Complex,// Complex
    Mod,    // Mod

    I,      // i (sqrt(-1))
    E,      // e (constant)
    Pi,     // pi (constant)

    Identifier(String),
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

    // TODO: >, <, >=, <=

    Arrow,    // ->
    FatArrow, // =>
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Carrot,   // ^
    Bar,      // |
    Ampersand // &
    LogicNot  // ~

    Unknown(char),
    InvalidNum(String),
}