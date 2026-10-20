//! Helper nodes for `match` and `for`.

use gradi_core::span::Span;

use super::{BinaryOp, Expr};
use crate::{ident::Ident, pat::Pat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    pub pat: Pat,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

/// The parenthesised part of a `for` loop.
///
/// ```text
/// for (0..10) { ... }                  // Anonymous
/// for (i in 0..10) { ... }             // Named, no step
/// for (i in 0..10; i += 2) { ... }     // Named, with step
/// ```
/// A step needs a named loop variable, so `Anonymous` has no step field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForHeader {
    Anonymous {
        iterable: Box<Expr>,
    },
    Named {
        binding: Ident,
        iterable: Box<Expr>,
        step: Option<ForStep>,
    },
}

/// `i += 2`. The parser checks that the target is the loop binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStep {
    pub target: Ident,
    pub op: BinaryOp,
    pub amount: Box<Expr>,
    pub span: Span,
}
