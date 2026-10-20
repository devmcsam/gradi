//! Expressions.

mod control_flow;
mod ops;

pub use control_flow::{ForHeader, ForStep, MatchArm};
pub use ops::{AddrKind, Associativity, BinaryOp, RangeLimits, UnaryOp};

use gradi_core::span::Span;

use crate::{
    block::Block,
    ident::Ident,
    literal::Literal,
    mutability::Mutability,
    node_id::NodeId,
    path::{Path, PathSegment},
    ty::Type,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub const fn new(id: NodeId, kind: ExprKind, span: Span) -> Self {
        Self { id, kind, span }
    }

    /// Block-like expressions can be used as statements without a trailing `;`.
    pub fn is_block_like(&self) -> bool {
        matches!(
            self.kind,
            ExprKind::Block(_)
                | ExprKind::Unsafe(_)
                | ExprKind::If { .. }
                | ExprKind::Match { .. }
                | ExprKind::While { .. }
                | ExprKind::Loop { .. }
                | ExprKind::For { .. }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Lit(Literal),
    Path(Path),
    /// `(expr)`, kept so formatters can round-trip the source.
    Paren(Box<Expr>),

    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// `lhs = rhs`
    Assign {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// `lhs += rhs` and friends.
    AssignOp {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// `&x`, `&mut x`, or a pointer-producing form. See [`AddrKind`].
    AddrOf {
        kind: AddrKind,
        mutability: Mutability,
        operand: Box<Expr>,
    },
    /// `*x`
    Deref(Box<Expr>),
    /// `expr as Type`
    Cast {
        expr: Box<Expr>,
        ty: Box<Type>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    MethodCall {
        receiver: Box<Expr>,
        segment: PathSegment,
        args: Vec<Expr>,
    },
    /// `base.field`
    Field {
        base: Box<Expr>,
        field: Ident,
    },
    /// `base.0`
    TupleField {
        base: Box<Expr>,
        index: u32,
    },
    /// `base[index]`
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    /// `a..b`, `a..=b`, `..`, `a..`, `..b`
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        limits: RangeLimits,
    },

    Tuple(Vec<Expr>),
    /// `[a, b, c]`
    Array(Vec<Expr>),
    /// `[elem; len]`
    ArrayRepeat {
        elem: Box<Expr>,
        len: Box<Expr>,
    },
    /// `Point { x: 1, y: 2 }`
    StructLit {
        path: Path,
        fields: Vec<FieldInit>,
    },

    Block(Box<Block>),
    Unsafe(Box<Block>),
    If {
        cond: Box<Expr>,
        then_branch: Box<Block>,
        else_branch: Option<Box<Expr>>,
    },
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Block>,
    },
    Loop {
        body: Box<Block>,
    },
    For {
        header: ForHeader,
        body: Box<Block>,
    },
    Break(Option<Box<Expr>>),
    Continue,
    Return(Option<Box<Expr>>),
}

/// `name: value` in a struct literal. Shorthand `name` is desugared to `name: name`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldInit {
    pub ident: Ident,
    pub value: Expr,
    pub span: Span,
}
