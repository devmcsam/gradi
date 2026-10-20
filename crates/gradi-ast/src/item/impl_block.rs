//! Unified `impl` blocks.
//!
//! Gradi lets trait implementations live *inside* the inherent `impl` block,
//! so generics are declared once:
//!
//! ```text
//! impl<T: Bar> Foo<T> {
//!     fn new() -> Self { ... }
//!
//!     impl Add { fn add(self, rhs: Self) -> Self { ... } }
//!     impl Sub { fn sub(self, rhs: Self) -> Self { ... } }
//! }
//! ```

use gradi_core::span::Span;

use super::AssocItem;
use crate::{
    generics::{Generics, WherePredicate},
    node_id::NodeId,
    path::Path,
    ty::Type,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplBlock {
    /// Shared by the inherent items and every nested trait impl.
    pub generics: Generics,
    pub self_ty: Type,
    pub members: Vec<ImplMember>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplMember {
    /// An inherent method or constant.
    Assoc(AssocItem),
    /// A nested trait implementation for the outer `self_ty`.
    Trait(TraitImpl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitImpl {
    pub id: NodeId,
    pub trait_ref: Path,
    /// Extra bounds that apply only to this trait impl.
    pub where_clause: Vec<WherePredicate>,
    pub items: Vec<AssocItem>,
    pub span: Span,
}
