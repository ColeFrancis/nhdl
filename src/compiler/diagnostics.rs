// Copyright 2026 Cole Francis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # error
//!
//! Defines error types from the lexer, parser, and semantic analysis
//!
//! ## Invariants
//!
//! - All compiler errors will be found in here
//!
//! Author: Cole Francis

use super::token::TokenKind;

pub struct Diagnostics {
    errors: Vec<CompilerError>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    pub fn error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    pub fn errors(&self) -> &[CompilerError] {
        &self.errors
    }
}

#[derive(Debug)]
pub enum CompilerError {
    // Lexer
    UnknownToken {
        lexeme: String,
        span: Span,
    },

    InvalidNum {
        lexeme: String,
        span: Span,
    },

    // Parser
    UnexpectedToken {
        expected: Vec<Expected>,
        found: TokenKind,
        span: Span,
    },
}

#[derive(Debug)]
pub enum Expected {
    Token(TokenKind),
    Expr,
    Ident,
    BoolLiteral,
    IntLiteral,
    RealLiteral,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}