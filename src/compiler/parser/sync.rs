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

//! # sync
//!
//! Handles recovery after detecting an error in parsing
//!
//! ## Invariants
//!
//! - 
//!
//! Author: Cole Francis

use super::Parser;
use crate::compiler::token::{Token, TokenKind};
use crate::compiler::ast::*;

pub enum SyncRule {
    Item, // top level
    Inst, // inside net_t
    Statement,
    Expr,
}

impl<'a> Parser<'a> {
    // Never consume a } that belongs to the caller's block.
    // If you skip into a nested {...} block, skip the entire block before trying to resume.
    // Only recover at tokens that are valid starts of constructs at the current nesting level
    pub(super) fn sync(&mut self, rule: SyncRule) {
        match rule {
            SyncRule::Item => self.sync_item(),

            SyncRule::Inst => self.sync_inst(),

            SyncRule::Statement => self.sync_statement(),

            SyncRule::Expr => self.sync_expr(),
        }
    }

    fn sync_item(&mut self) {

    }

    fn sync_inst(&mut self) {

    }

    fn sync_statement(&mut self) {

    }

    fn sync_expr(&mut self) {

    }
}