//! Functions and methods.

use gradi_core::span::Span;

use crate::{
    block::Block, generics::Generics, ident::Ident, mutability::Mutability, node_id::NodeId,
    pat::Pat, ty::Type,
};

/// A function or method. `body` is `None` for declarations without a default
/// body, such as trait methods that implementors must provide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub sig: FnSig,
    pub body: Option<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnSig {
    pub is_unsafe: bool,
    pub name: Ident,
    pub generics: Generics,
    /// `self`, `mut self`, `&self`, `&mut self`
    pub receiver: Option<SelfParam>,
    pub params: Vec<Param>,
    /// `None` means the function returns unit.
    pub ret_ty: Option<Type>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfParam {
    pub kind: SelfKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfKind {
    /// `self` / `mut self`
    Value(Mutability),
    /// `&self` / `&mut self`
    Ref(Mutability),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub id: NodeId,
    pub pat: Pat,
    pub ty: Type,
    pub span: Span,
}
