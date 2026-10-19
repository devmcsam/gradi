//! The interner is a data structure that maps strings to unique identifiers.
use lasso::{Rodeo, Spur};

pub trait SymbolInterner {
    type Symbol: Eq + Copy + std::hash::Hash;

    fn intern(&mut self, string: &str) -> Self::Symbol;
    fn resolve(&self, symbol: Self::Symbol) -> &str;
}

impl SymbolInterner for Rodeo<Spur> {
    type Symbol = Spur;

    #[inline]
    fn intern(&mut self, string: &str) -> Self::Symbol {
        self.get_or_intern(string)
    }

    #[inline]
    fn resolve(&self, symbol: Self::Symbol) -> &str {
        Rodeo::<Spur>::resolve(self, &symbol)
    }
}
