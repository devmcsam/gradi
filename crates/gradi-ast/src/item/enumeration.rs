//! Enum definitions.

use gradi_core::span::Span;

use super::structure::FieldDef;
use crate::{expr::Expr, generics::Generics, ident::Ident, node_id::NodeId, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDef {
    pub name: Ident,
    pub generics: Generics,
    pub variants: Vec<Variant>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub id: NodeId,
    pub name: Ident,
    pub kind: VariantKind,
    pub discriminant: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantKind {
    /// `Red`
    Unit,
    /// `Rgb(u8, u8, u8)`
    Tuple(Vec<Type>),
    /// `Point { x: i32, y: i32 }`
    Struct(Vec<FieldDef>),
}
