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
//! Last Updated: 06/16/2026

struct Program {
    items: Vec<Item>,
}

enum Item {
    Ent(EntDecl),
    Rel(RelDecl),
    Net(NetDecl),
}

struct EntDecl {
    name: String,
}

struct RelDecl {
    name: String,
    ports: Vec<Port>,
    return_type: Type,
    body: Block,
}

struct Parameter {
    name: String,
    type: Type,
}

struct NetDecl {
    name: String,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    inits: Vec<Port>,
    members: Vec<ModuleMembers>,
}