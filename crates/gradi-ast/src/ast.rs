//! The root of the syntax tree.

use gradi_core::span::Span;

use crate::item::Item;

/// The AST or abstract syntax tree for the Gradi programming language.
///
/// One `Ast` corresponds to one parsed source file.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Ast {
    pub items: Vec<Item>,
    pub span: Span,
}
