//! Unique ids for AST nodes, used as keys for side tables in later passes.

/// A unique id for an AST node within one compilation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    /// Placeholder for synthesized nodes and tests.
    pub const DUMMY: NodeId = NodeId(u32::MAX);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Hands out fresh [`NodeId`]s. The parser owns one of these.
#[derive(Debug, Default)]
pub struct NodeIdGen {
    next: u32,
}

impl NodeIdGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_id(&mut self) -> NodeId {
        assert!(self.next != u32::MAX, "NodeId space exhausted");
        let id = NodeId(self.next);
        self.next += 1;
        id
    }
}
