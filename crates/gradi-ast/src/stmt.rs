//! Statements.

use gradi_core::span::Span;

use crate::{expr::Expr, item::Item, node_id::NodeId, pat::Pat, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
    pub id: NodeId,
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub const fn new(id: NodeId, kind: StmtKind, span: Span) -> Self {
        Self { id, kind, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind {
    /// `let mut x: T = init;`
    Let(Box<LetStmt>),
    /// An item declared inside a function body.
    Item(Box<Item>),
    /// A block-like expression used as a statement without `;` (`if`, `loop`, ...).
    Expr(Box<Expr>),
    /// `expr;`
    Semi(Box<Expr>),
    /// A stray `;`
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStmt {
    /// `mut` lives on the binding pattern.
    pub pat: Pat,
    /// Required for now; `Option` so inference can be added later without AST changes.
    pub ty: Option<Type>,
    pub init: Option<Box<Expr>>,
}
