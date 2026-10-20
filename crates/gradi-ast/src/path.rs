//! Paths such as `a::b::C<T>` or `Graph<Node>::new`.

use gradi_core::span::Span;

use crate::{expr::Expr, ident::Ident, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    /// The path's only segment, if it has exactly one and no generic args.
    pub fn as_ident(&self) -> Option<&Ident> {
        match self.segments.as_slice() {
            [seg] if seg.args.is_none() => Some(&seg.ident),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
    pub ident: Ident,
    pub args: Option<GenericArgs>,
}

/// `<A, B, 3>` in a path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericArgs {
    pub args: Vec<GenericArg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericArg {
    Type(Type),
    Const(Box<Expr>),
}
