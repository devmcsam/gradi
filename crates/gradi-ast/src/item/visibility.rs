//! Item visibility.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Visibility {
    /// No `pub`.
    #[default]
    Inherited,
    Public,
}
