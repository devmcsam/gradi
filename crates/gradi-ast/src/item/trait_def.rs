//! Trait definitions.

use gradi_core::span::Span;

use super::AssocItem;
use crate::{
    generics::{Generics, TraitBound},
    ident::Ident,
};

/// Traits contain functions and constants that implementors must provide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitDef {
    pub name: Ident,
    pub generics: Generics,
    /// `trait A: B + C`
    pub supertraits: Vec<TraitBound>,
    pub items: Vec<AssocItem>,
    pub span: Span,
}
