//! `const NAME: Type = value;`

use gradi_core::span::Span;

use crate::{expr::Expr, ident::Ident, ty::Type};

/// `value` is `None` for trait constants that implementors must define.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstItem {
    pub name: Ident,
    /// Required for now.
    pub ty: Type,
    pub value: Option<Expr>,
    pub span: Span,
}
