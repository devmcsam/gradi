//! Top-level and nested declarations.

pub mod assoc;
pub mod constant;
pub mod enumeration;
pub mod function;
pub mod impl_block;
pub mod module;
pub mod structure;
pub mod trait_def;
pub mod type_decl;
pub mod use_decl;
pub mod visibility;

pub use assoc::{AssocItem, AssocItemKind};
pub use constant::ConstItem;
pub use enumeration::{EnumDef, Variant, VariantKind};
pub use function::{FnSig, Function, Param, SelfKind, SelfParam};
pub use impl_block::{ImplBlock, ImplMember, TraitImpl};
pub use module::ModDecl;
pub use structure::{FieldDef, StructDef};
pub use trait_def::TraitDef;
pub use type_decl::TypeDecl;
pub use use_decl::{UseDecl, UseTree, UseTreeKind};
pub use visibility::Visibility;

use gradi_core::span::Span;

use crate::{ident::Ident, node_id::NodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: NodeId,
    pub vis: Visibility,
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
    Fn(Box<Function>),
    Const(Box<ConstItem>),
    TypeDecl(Box<TypeDecl>),
    Struct(Box<StructDef>),
    Enum(Box<EnumDef>),
    Trait(Box<TraitDef>),
    Impl(Box<ImplBlock>),
    Mod(Box<ModDecl>),
    Use(Box<UseDecl>),
}

impl ItemKind {
    /// The declared name, for items that have one.
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            ItemKind::Fn(i) => Some(&i.sig.name),
            ItemKind::Const(i) => Some(&i.name),
            ItemKind::TypeDecl(i) => Some(&i.name),
            ItemKind::Struct(i) => Some(&i.name),
            ItemKind::Enum(i) => Some(&i.name),
            ItemKind::Trait(i) => Some(&i.name),
            ItemKind::Mod(i) => Some(&i.name),
            ItemKind::Impl(_) | ItemKind::Use(_) => None,
        }
    }
}
