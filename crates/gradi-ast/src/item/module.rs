//! Module declarations.

use gradi_core::span::Span;

use super::Item;
use crate::ident::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModDecl {
    pub name: Ident,
    /// `Some` for inline `mod a { ... }`, `None` for `mod a;` (loaded from another file).
    pub items: Option<Vec<Item>>,
    pub span: Span,
}
