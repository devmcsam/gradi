//! Type expressions (the syntax of types, not resolved types).

use gradi_core::span::Span;

use crate::{expr::Expr, mutability::Mutability, node_id::NodeId, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub id: NodeId,
    pub kind: TypeKind,
    pub span: Span,
}

impl Type {
    pub const fn new(id: NodeId, kind: TypeKind, span: Span) -> Self {
        Self { id, kind, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    /// `i32`, `Vec<T>`, `Handle<Node>`
    Path(Path),
    /// `&T` / `&mut T`. Always valid.
    Ref {
        mutability: Mutability,
        inner: Box<Type>,
    },
    /// `*T` / `*mut T`. May point anywhere.
    Ptr {
        mutability: Mutability,
        inner: Box<Type>,
    },
    /// `(A, B)`; the empty tuple is unit.
    Tuple(Vec<Type>),
    /// `[T; N]`
    Array { elem: Box<Type>, len: Box<Expr> },
    /// `[T]`
    Slice(Box<Type>),
    /// `fn(A, B) -> C`
    Fn {
        params: Vec<Type>,
        ret: Option<Box<Type>>,
    },
    /// `!`
    Never,
    /// `_`: ask the compiler to infer this type.
    Infer,
}
