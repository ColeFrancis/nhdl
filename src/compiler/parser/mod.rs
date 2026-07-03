mod core;
mod ent;
mod rel;
mod net;
mod expr;

use crate::compiler::token::Token;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}