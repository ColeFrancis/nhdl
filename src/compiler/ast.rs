//! # ast
//!
//! Holds the structures used in creating the ast
//!
//! ## Invariants
//!
//! - Grammar shall be obeyed. It is the source of truth.
//!
//! Author: Cole Francis
//!
//! Last Updated: 06/20/2026

struct Program {
    items: Vec<Item>,
}

enum Item {
    Ent(EntDecl),
    Rel(RelDecl),
    Net(NetDecl),
}

////////////////////////////////////////////////////////////////////////////////
/// Common AST elements
////////////////////////////////////////////////////////////////////////////////

type Ident = String;

enum Type {
    Bool,
    Int,
    Real,
    Mod(i64),
    CustomType(Ident),
}

enum Expr {
    Literal(Literal),
    Ident(Ident),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Tuple(TupleExpr),
    Match(MatchExpr),
    Sample(SampleExpr),
}

enum Literal {
    Bool(bool),
    Int(i64),
    Real(f64),
    Const(MathConst),
}

enum MathConst {
    E,
    Pi,
}

struct UnaryExpr {
    op: UnaryOp,
    expr: Box<Expr>,
}

enum UnaryOp {
    Neg,    // -
    Not,    // !
    BitNot, // ~
}

struct BinaryExpr {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
}

enum BinaryOp {
    Comp(CompOp),
    Arith(ArithOp),
}

enum CompOp {
    Lt,         // <
    Gt,         // >
    Le,         // <=
    Ge,         // >=
}

enum ArithOp {
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    Pow,        // ^
}

struct TupleExpr {
    elements: Vec<Expr>,
}

struct MatchExpr {
    scrutinee: Box<Expr>,
    arms: Vec<MatchArm>,
}

struct MatchArm {
    pattern: Vec<SimplePattern>,
    expr: Expr,
}

enum SimplePattern {
    Default,
    Literal(Literal),
    Ident(Ident),
    Tuple(Vec<SimplePattern>),
    Comparison(ComparisonPattern),
}

struct ComparisonPattern {
    op: CompOp,
    expr: Box<Expr>,
}

struct SampleExpr {
    arms: Vec<SampleArm>,
}

struct SampleArm {
    prob: Expr,
    expr: Expr,
}

struct Param {
    name: Ident,
    param_type: Type,
}

////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////

struct EntDecl {
    name: Ident,
    expr: EntExpr,
}

enum EntExpr {
    Type(Type),
    SetEnt(Vec<Ident>),
}

////////////////////////////////////////////////////////////////////////////////
/// Relations
////////////////////////////////////////////////////////////////////////////////

struct RelDecl {
    name: Ident,
    params: Vec<Param>,
    return_type: Type,
    body: RelBody,
}

enum RelBody {
    Expr(Expr),
    Block(BlockExpr),
}

struct BlockExpr {
    statements: Vec<LetStatement>,
    expr: Expr,
}

struct LetStatement {
    name: Ident,
    expr: Expr,
}

////////////////////////////////////////////////////////////////////////////////
/// Networks
////////////////////////////////////////////////////////////////////////////////

struct NetDecl {
    name: Ident,
    items: Vec<NetItem>,
}

enum NetItem {
    Input(Param),
    Output(Param),
    Init(NetInit),
    RelInst(RelInst),
}

struct NetInit {
    param: Param,
    val: Expr,
}

struct RelInst {
    asignee: Ident,
    rel: Ident,
    args: Vec<Expr>, 
}