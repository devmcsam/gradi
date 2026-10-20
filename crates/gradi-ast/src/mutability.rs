//! Mutability markers shared by bindings, references and pointers.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mutability {
    #[default]
    Not,
    Mut,
}

impl Mutability {
    pub const fn is_mut(self) -> bool {
        matches!(self, Mutability::Mut)
    }
}
