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

//! # ann_ast
//!
//! Holds the structures used in creating the annotated ast
//!
//! ## Invariants
//!
//! - Grammar shall be obeyed. It is the source of truth.
//!
//! Author: Cole Francis

// WARNING: may need additional info like types in binary expressions

use super::symbol::SymbolId;

#[derive(PartialEq, Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(PartialEq, Debug)]
pub enum Item {
    Let(LetStatement),
    Ent(EntType),
    Rel(RelType),
    Net(Net),
    Error,
}

////////////////////////////////////////////////////////////////////////////////
/// Common AST elements
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub enum Type {
    Bool,
    Impulse,
    Int,
    Real,
    Mod(i64),
    CustomType(SymbolId),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(SymbolId),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Tuple(Vec<Expr>),
    Match(MatchExpr),
    Sample(Vec<SampleArm>),
    Error,
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Real(f64),
}

#[derive(PartialEq, Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}

#[derive(PartialEq, Debug)]
pub enum UnaryOp {
    Neg,    // -
    BitNot, // ~
}

#[derive(PartialEq, Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOp {
    Lt,         // <
    Gt,         // >
    Le,         // <=
    Ge,         // >=
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    Pow,        // ^
}

#[derive(PartialEq, Debug)]
pub enum CompOp {
    Lt,         // <
    Gt,         // >
    Le,         // <=
    Ge,         // >=
}

#[derive(PartialEq, Debug)]
pub struct MatchExpr {
    pub scrutinee: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

#[derive(PartialEq, Debug)]
pub struct MatchArm {
    pub pattern: Vec<SimplePattern>,
    pub expr: Expr,
}

#[derive(PartialEq, Debug)]
pub enum SimplePattern {
    Default,
    Literal(Literal),
    Ident(SymbolId),
    Tuple(Vec<SimplePattern>),
    Comparison(ComparisonPattern),
}

#[derive(PartialEq, Debug)]
pub struct ComparisonPattern {
    pub op: CompOp,
    pub expr: Box<Expr>,
}

#[derive(PartialEq, Debug)]
pub struct SampleArm {
    pub prob: Prob,
    pub expr: Expr,
}

#[derive(PartialEq, Debug)]
pub enum Prob {
    Default,
    Expr(Expr),
}

#[derive(PartialEq, Debug)]
pub struct Param {
    pub symbol: SymbolId,
    pub param_type: Type,
}

////////////////////////////////////////////////////////////////////////////////
/// Statements
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(LetStatement),
    Error,
}

#[derive(PartialEq, Debug)]
pub struct LetStatement {
    pub symbol: SymbolId,
    pub expr: Expr,
}

////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub struct EntType {
    pub symbol: SymbolId,
    pub expr: EntExpr,
}

#[derive(PartialEq, Debug)]
pub enum EntExpr {
    Type(Type),
    SetEnt(Vec<SymbolId>),
}

////////////////////////////////////////////////////////////////////////////////
/// Relations
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub struct RelType {
    pub symbol: SymbolId,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: RelBody,
}

#[derive(PartialEq, Debug)]
pub enum RelBody {
    Expr(Expr),
    Block(BlockExpr),
}

#[derive(PartialEq, Debug)]
pub struct BlockExpr {
    pub statements: Vec<Statement>,
    pub expr: Expr,
}

////////////////////////////////////////////////////////////////////////////////
/// Networks
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub struct Net {
    pub symbol: SymbolId,
    pub items: Vec<NetItem>,
}

#[derive(PartialEq, Debug)]
pub enum NetItem {
    Input(Param),
    Output(Param),
    Init(EntInit),
    RelInst(RelInst),
    NetInst(NetInst),
    Error,
}

#[derive(PartialEq, Debug)]
pub struct EntInit {
    pub param: Param,
    pub val: Expr,
}

#[derive(PartialEq, Debug)]
pub struct RelInst {
    pub asignee: SymbolId,
    pub rel: SymbolId,
    pub args: Vec<SymbolId>, 
}

#[derive(PartialEq, Debug)]
pub struct NetInst {
    pub net: SymbolId,
    pub connections: Vec<Connection>,
}

#[derive(PartialEq, Debug)]
pub struct Connection {
    pub port: SymbolId,
    pub net: SymbolId,
}