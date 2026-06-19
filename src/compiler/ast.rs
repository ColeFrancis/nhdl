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
//! Last Updated: 06/18/2026

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

enum Type {
    Bool,
    Int,
    Real,
    Complex,
    Mod(i64),
    CustomType(String),
}

enum Expr {
    Literal(Literal),
    Ident(String),
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
    Complex,
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
    Neg,
    Not,
}

struct BinaryExpr {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
}

enum BinaryOp {
    Eq(EqOp),
    Comp(CompOp),
    Arith(ArithOp),
    Logic(LogicOp),
}

enum EqOp {
    Eq,         // ==
    Neq,        // !=
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
}

enum LogicOp {
    And,        // &
    Or,         // |
    LogicNot,   // ~
}

struct TupleExpr {
    elements: Vec<Expr>,
}

struct MatchExpr {
    sctutinee: Expr,
    arms: Vec<MatchArm>,
}

struct MatchArm {
    pattern: Vec<SimplePattern>,
    expr: Expr,
}

enum SimplePattern {
    Default,
    Literal,
    Ident(String),
    Tuple(Vec<SimplePattern>),
    Comparison(ComparisonPattern),
}

struct ComparisonPattern {
    op: CompOp,
    expr: Expr,
}

struct SampleExpr {
    arms: Vec<SampArm>,
}

struct SampArm {
    prob: Expr,
    expr: Expr,
}

struct Param {
    name: String,
    param_type: Type,
}

////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////

struct EntDecl {
    name: String,
    expr: EntExpr,
}

enum EntExpr {
    Type,
    SetEnt(Vec<Ident(String)>),
}

////////////////////////////////////////////////////////////////////////////////
/// Relations
////////////////////////////////////////////////////////////////////////////////

struct RelDecl {
    name: String,
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
    name: String,
    expr: Expr,
}

////////////////////////////////////////////////////////////////////////////////
/// Networks
////////////////////////////////////////////////////////////////////////////////

struct NetDecl {
    name: String,
    inputs: Vec<Param>,
    outputs: Vec<Param>,
    inits: Vec<NetInit>,
    assignments: Vec<NetAssignment>,
}

struct NetInit {
    param: Param,
    val: Expr,
}

struct: Port {
    name: String,
    port_type: Type,
}

struct NetAssignment {
    // TODO
}