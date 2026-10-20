//! `type Name = Existing;`

use gradi_core::span::Span;

use crate::{generics::Generics, ident::Ident, ty::Type};

/// Unlike Rust's `type`, a Gradi `type` declaration creates a *distinct* type
/// that is only represented as `aliased`. The AST just records the
/// declaration; the type checker decides what it means.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDecl {
    pub name: Ident,
    pub generics: Generics,
    pub aliased: Type,
    pub span: Span,
}
