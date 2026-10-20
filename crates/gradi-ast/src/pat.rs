//! Patterns, used by `let`, parameters and `match`.

use gradi_core::span::Span;

use crate::{expr::Expr, ident::Ident, mutability::Mutability, node_id::NodeId, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pat {
    pub id: NodeId,
    pub kind: PatKind,
    pub span: Span,
}

impl Pat {
    pub const fn new(id: NodeId, kind: PatKind, span: Span) -> Self {
        Self { id, kind, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatKind {
    /// `_`
    Wildcard,
    /// `x`, `mut x`, `x @ pat`
    Binding {
        mutability: Mutability,
        ident: Ident,
        subpattern: Option<Box<Pat>>,
    },
    /// A literal expression, possibly negated.
    Lit(Box<Expr>),
    /// `a..b`, `a..=b`, `a..`, `..b`
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        inclusive: bool,
    },
    Tuple(Vec<Pat>),
    /// A unit variant or constant, e.g. `Color::Red`.
    Path(Path),
    /// `Some(x)`
    TupleStruct {
        path: Path,
        elems: Vec<Pat>,
    },
    /// `Point { x, y: 0, .. }`
    Struct {
        path: Path,
        fields: Vec<FieldPat>,
        has_rest: bool,
    },
    /// `&pat` / `&mut pat`
    Ref {
        mutability: Mutability,
        inner: Box<Pat>,
    },
    /// `a | b | c`
    Or(Vec<Pat>),
}

/// A field in a struct pattern. Shorthand `x` is desugared to `x: x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPat {
    pub ident: Ident,
    pub pat: Pat,
    pub span: Span,
}
