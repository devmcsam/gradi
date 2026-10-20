//! Struct definitions.

use gradi_core::span::Span;

use super::Visibility;
use crate::{generics::Generics, ident::Ident, node_id::NodeId, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDef {
    pub name: Ident,
    pub generics: Generics,
    pub fields: Vec<FieldDef>,
    pub span: Span,
}

/// `name: Type`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    pub id: NodeId,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
    pub span: Span,
}
