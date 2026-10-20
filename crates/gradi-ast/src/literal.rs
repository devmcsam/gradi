//! Literal values.

use gradi_core::span::Span;
use gradi_interner::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Literal {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LitKind {
    /// `42`, `0xff`, `1_000u64`. The lexer has already parsed the digits.
    Int {
        value: u128,
        suffix: Option<Symbol>,
    },
    /// Kept as source text so the AST stays `Eq`/`Hash`. Parsed during lowering.
    Float {
        text: Symbol,
        suffix: Option<Symbol>,
    },
    Bool(bool),
    Char(char),
    /// String contents after escape processing.
    Str(Symbol),
}
