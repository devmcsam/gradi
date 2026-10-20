//! Identifiers.

use gradi_core::span::Span;
use gradi_interner::Symbol;

/// An interned name together with where it appeared in the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

impl Ident {
    pub const fn new(name: Symbol, span: Span) -> Self {
        Self { name, span }
    }
}
