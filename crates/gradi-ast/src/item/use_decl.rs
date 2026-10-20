//! Imports.

use gradi_core::span::Span;

use crate::{ident::Ident, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseDecl {
    pub tree: UseTree,
    pub span: Span,
}

/// `a::b`, `a::b as c`, `a::*`, `a::{b, c::d}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseTree {
    pub prefix: Path,
    pub kind: UseTreeKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseTreeKind {
    Simple { rename: Option<Ident> },
    Glob,
    Nested(Vec<UseTree>),
}
