//! Blocks: `{ stmts; tail }`.

use gradi_core::span::Span;

use crate::{expr::Expr, node_id::NodeId, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub id: NodeId,
    pub stmts: Vec<Stmt>,
    /// The final expression without a trailing `;`, which is the block's value.
    pub tail: Option<Box<Expr>>,
    pub span: Span,
}

impl Block {
    pub const fn new(id: NodeId, stmts: Vec<Stmt>, tail: Option<Box<Expr>>, span: Span) -> Self {
        Self {
            id,
            stmts,
            tail,
            span,
        }
    }
}
