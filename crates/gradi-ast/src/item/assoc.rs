//! Items that can appear inside a trait or an impl block.

use gradi_core::span::Span;

use super::{ConstItem, Function, Visibility};
use crate::node_id::NodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssocItem {
    pub id: NodeId,
    pub vis: Visibility,
    pub kind: AssocItemKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssocItemKind {
    Fn(Box<Function>),
    Const(Box<ConstItem>),
}
