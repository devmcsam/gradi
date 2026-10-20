//! Generic parameters, trait bounds and where clauses.

use gradi_core::span::Span;

use crate::{expr::Expr, ident::Ident, node_id::NodeId, path::Path, ty::Type};

/// `<T: Bar, const N: usize> where ...`. Empty when the item is not generic.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Generics {
    pub params: Vec<GenericParam>,
    pub where_clause: Vec<WherePredicate>,
    pub span: Span,
}

impl Generics {
    pub fn is_empty(&self) -> bool {
        self.params.is_empty() && self.where_clause.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParam {
    pub id: NodeId,
    pub ident: Ident,
    pub kind: GenericParamKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericParamKind {
    Type {
        bounds: Vec<TraitBound>,
        default: Option<Type>,
    },
    Const {
        ty: Type,
        default: Option<Box<Expr>>,
    },
}

/// A reference to a trait used as a bound, e.g. `Add<Output = T>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitBound {
    pub path: Path,
    pub span: Span,
}

/// `Type: BoundA + BoundB`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WherePredicate {
    pub bounded_ty: Type,
    pub bounds: Vec<TraitBound>,
    pub span: Span,
}
