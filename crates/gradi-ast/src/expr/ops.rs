//! Operators and the precedence information the parser needs.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    /// `-x`
    Neg,
    /// `!x`
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    /// `&&` (short-circuiting)
    And,
    /// `||` (short-circuiting)
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    Left,
    /// Cannot be chained: `a < b < c` is an error.
    None,
}

impl BinaryOp {
    pub const fn as_str(self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Rem => "%",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
        }
    }

    /// Higher binds tighter. Assignment and ranges are below all of these.
    pub const fn precedence(self) -> u8 {
        match self {
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => 10,
            BinaryOp::Add | BinaryOp::Sub => 9,
            BinaryOp::Shl | BinaryOp::Shr => 8,
            BinaryOp::BitAnd => 7,
            BinaryOp::BitXor => 6,
            BinaryOp::BitOr => 5,
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => 4,
            BinaryOp::And => 3,
            BinaryOp::Or => 2,
        }
    }

    pub const fn associativity(self) -> Associativity {
        if self.is_comparison() {
            Associativity::None
        } else {
            Associativity::Left
        }
    }

    pub const fn is_comparison(self) -> bool {
        matches!(
            self,
            BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
        )
    }

    /// `&&` and `||` do not always evaluate their right-hand side.
    pub const fn is_lazy(self) -> bool {
        matches!(self, BinaryOp::And | BinaryOp::Or)
    }

    /// Whether `op=` is a valid compound assignment (`+=`, `<<=`, ...).
    pub const fn supports_compound_assign(self) -> bool {
        !self.is_comparison() && !self.is_lazy()
    }
}

/// Which kind of address an address-of expression produces.
///
/// Gradi has not yet decided how pointers are created in expressions,
/// since `*x` already means dereference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddrKind {
    Ref,
    Ptr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RangeLimits {
    /// `a..b`
    HalfOpen,
    /// `a..=b`
    Closed,
}
