//! The AST and operations defined on said AST for the Gradi programming language.

pub mod ast;
pub mod block;
pub mod expr;
pub mod generics;
pub mod ident;
pub mod item;
pub mod literal;
pub mod mutability;
pub mod node_id;
pub mod pat;
pub mod path;
pub mod stmt;
pub mod ty;

/// The AST or abstract syntax tree for the Gradi programming language.
pub enum Ast {}
