//! The interner is a data structure that maps strings to unique identifiers.
use lasso::{Rodeo, Spur};

pub type Interner = Rodeo<Symbol>;
pub type Symbol = Spur;

pub trait SymbolInterner {
    type Symbol: Eq + Copy + std::hash::Hash;

    fn intern(&mut self, string: &str) -> Self::Symbol;
    fn resolve(&self, symbol: Self::Symbol) -> &str;
}

impl SymbolInterner for Interner {
    type Symbol = Spur;

    #[inline]
    fn intern(&mut self, string: &str) -> Self::Symbol {
        self.get_or_intern(string)
    }

    #[inline]
    fn resolve(&self, symbol: Self::Symbol) -> &str {
        Interner::resolve(self, &symbol)
    }
}
