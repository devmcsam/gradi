//! Read-only traversal of the AST.
//!
//! Override the `visit_*` methods you care about and call the matching
//! `walk_*` function to keep descending.

use crate::{
    ast::Ast,
    block::Block,
    expr::{Expr, ExprKind, ForHeader},
    generics::{GenericParam, GenericParamKind, Generics, TraitBound, WherePredicate},
    ident::Ident,
    item::{
        AssocItem, AssocItemKind, ConstItem, EnumDef, FieldDef, Function, ImplBlock, ImplMember,
        Item, ItemKind, ModDecl, Param, StructDef, TraitDef, TraitImpl, TypeDecl, Variant,
        VariantKind,
    },
    literal::Literal,
    pat::{Pat, PatKind},
    path::{GenericArg, GenericArgs, Path, PathSegment},
    stmt::{Stmt, StmtKind},
    ty::{Type, TypeKind},
};

pub trait Visitor<'ast>: Sized {
    fn visit_ast(&mut self, ast: &'ast Ast) {
        walk_ast(self, ast)
    }
    fn visit_item(&mut self, item: &'ast Item) {
        walk_item(self, item)
    }
    fn visit_fn(&mut self, f: &'ast Function) {
        walk_fn(self, f)
    }
    fn visit_param(&mut self, param: &'ast Param) {
        walk_param(self, param)
    }
    fn visit_const(&mut self, c: &'ast ConstItem) {
        walk_const(self, c)
    }
    fn visit_type_decl(&mut self, t: &'ast TypeDecl) {
        walk_type_decl(self, t)
    }
    fn visit_struct(&mut self, s: &'ast StructDef) {
        walk_struct(self, s)
    }
    fn visit_field_def(&mut self, f: &'ast FieldDef) {
        walk_field_def(self, f)
    }
    fn visit_enum(&mut self, e: &'ast EnumDef) {
        walk_enum(self, e)
    }
    fn visit_variant(&mut self, v: &'ast Variant) {
        walk_variant(self, v)
    }
    fn visit_trait(&mut self, t: &'ast TraitDef) {
        walk_trait(self, t)
    }
    fn visit_impl(&mut self, i: &'ast ImplBlock) {
        walk_impl(self, i)
    }
    fn visit_trait_impl(&mut self, i: &'ast TraitImpl) {
        walk_trait_impl(self, i)
    }
    fn visit_assoc_item(&mut self, i: &'ast AssocItem) {
        walk_assoc_item(self, i)
    }
    fn visit_mod(&mut self, m: &'ast ModDecl) {
        walk_mod(self, m)
    }
    fn visit_generics(&mut self, g: &'ast Generics) {
        walk_generics(self, g)
    }
    fn visit_generic_param(&mut self, p: &'ast GenericParam) {
        walk_generic_param(self, p)
    }
    fn visit_where_predicate(&mut self, p: &'ast WherePredicate) {
        walk_where_predicate(self, p)
    }
    fn visit_trait_bound(&mut self, b: &'ast TraitBound) {
        walk_trait_bound(self, b)
    }
    fn visit_path(&mut self, p: &'ast Path) {
        walk_path(self, p)
    }
    fn visit_path_segment(&mut self, s: &'ast PathSegment) {
        walk_path_segment(self, s)
    }
    fn visit_generic_args(&mut self, a: &'ast GenericArgs) {
        walk_generic_args(self, a)
    }
    fn visit_ty(&mut self, t: &'ast Type) {
        walk_ty(self, t)
    }
    fn visit_pat(&mut self, p: &'ast Pat) {
        walk_pat(self, p)
    }
    fn visit_expr(&mut self, e: &'ast Expr) {
        walk_expr(self, e)
    }
    fn visit_block(&mut self, b: &'ast Block) {
        walk_block(self, b)
    }
    fn visit_stmt(&mut self, s: &'ast Stmt) {
        walk_stmt(self, s)
    }
    fn visit_ident(&mut self, _ident: &'ast Ident) {}
    fn visit_literal(&mut self, _lit: &'ast Literal) {}
}

pub fn walk_ast<'ast, V: Visitor<'ast>>(v: &mut V, ast: &'ast Ast) {
    for item in &ast.items {
        v.visit_item(item);
    }
}

pub fn walk_item<'ast, V: Visitor<'ast>>(v: &mut V, item: &'ast Item) {
    match &item.kind {
        ItemKind::Fn(i) => v.visit_fn(i),
        ItemKind::Const(i) => v.visit_const(i),
        ItemKind::TypeDecl(i) => v.visit_type_decl(i),
        ItemKind::Struct(i) => v.visit_struct(i),
        ItemKind::Enum(i) => v.visit_enum(i),
        ItemKind::Trait(i) => v.visit_trait(i),
        ItemKind::Impl(i) => v.visit_impl(i),
        ItemKind::Mod(i) => v.visit_mod(i),
        ItemKind::Use(_) => {}
    }
}

pub fn walk_fn<'ast, V: Visitor<'ast>>(v: &mut V, f: &'ast Function) {
    v.visit_ident(&f.sig.name);
    v.visit_generics(&f.sig.generics);
    for param in &f.sig.params {
        v.visit_param(param);
    }
    if let Some(ret) = &f.sig.ret_ty {
        v.visit_ty(ret);
    }
    if let Some(body) = &f.body {
        v.visit_block(body);
    }
}

pub fn walk_param<'ast, V: Visitor<'ast>>(v: &mut V, param: &'ast Param) {
    v.visit_pat(&param.pat);
    v.visit_ty(&param.ty);
}

pub fn walk_const<'ast, V: Visitor<'ast>>(v: &mut V, c: &'ast ConstItem) {
    v.visit_ident(&c.name);
    v.visit_ty(&c.ty);
    if let Some(value) = &c.value {
        v.visit_expr(value);
    }
}

pub fn walk_type_decl<'ast, V: Visitor<'ast>>(v: &mut V, t: &'ast TypeDecl) {
    v.visit_ident(&t.name);
    v.visit_generics(&t.generics);
    v.visit_ty(&t.aliased);
}

pub fn walk_struct<'ast, V: Visitor<'ast>>(v: &mut V, s: &'ast StructDef) {
    v.visit_ident(&s.name);
    v.visit_generics(&s.generics);
    for field in &s.fields {
        v.visit_field_def(field);
    }
}

pub fn walk_field_def<'ast, V: Visitor<'ast>>(v: &mut V, f: &'ast FieldDef) {
    v.visit_ident(&f.name);
    v.visit_ty(&f.ty);
}

pub fn walk_enum<'ast, V: Visitor<'ast>>(v: &mut V, e: &'ast EnumDef) {
    v.visit_ident(&e.name);
    v.visit_generics(&e.generics);
    for variant in &e.variants {
        v.visit_variant(variant);
    }
}

pub fn walk_variant<'ast, V: Visitor<'ast>>(v: &mut V, variant: &'ast Variant) {
    v.visit_ident(&variant.name);
    match &variant.kind {
        VariantKind::Unit => {}
        VariantKind::Tuple(tys) => tys.iter().for_each(|t| v.visit_ty(t)),
        VariantKind::Struct(fields) => fields.iter().for_each(|f| v.visit_field_def(f)),
    }
    if let Some(d) = &variant.discriminant {
        v.visit_expr(d);
    }
}

pub fn walk_trait<'ast, V: Visitor<'ast>>(v: &mut V, t: &'ast TraitDef) {
    v.visit_ident(&t.name);
    v.visit_generics(&t.generics);
    for bound in &t.supertraits {
        v.visit_trait_bound(bound);
    }
    for item in &t.items {
        v.visit_assoc_item(item);
    }
}

pub fn walk_impl<'ast, V: Visitor<'ast>>(v: &mut V, i: &'ast ImplBlock) {
    v.visit_generics(&i.generics);
    v.visit_ty(&i.self_ty);
    for member in &i.members {
        match member {
            ImplMember::Assoc(item) => v.visit_assoc_item(item),
            ImplMember::Trait(t) => v.visit_trait_impl(t),
        }
    }
}

pub fn walk_trait_impl<'ast, V: Visitor<'ast>>(v: &mut V, i: &'ast TraitImpl) {
    v.visit_path(&i.trait_ref);
    for pred in &i.where_clause {
        v.visit_where_predicate(pred);
    }
    for item in &i.items {
        v.visit_assoc_item(item);
    }
}

pub fn walk_assoc_item<'ast, V: Visitor<'ast>>(v: &mut V, i: &'ast AssocItem) {
    match &i.kind {
        AssocItemKind::Fn(f) => v.visit_fn(f),
        AssocItemKind::Const(c) => v.visit_const(c),
    }
}

pub fn walk_mod<'ast, V: Visitor<'ast>>(v: &mut V, m: &'ast ModDecl) {
    v.visit_ident(&m.name);
    if let Some(items) = &m.items {
        for item in items {
            v.visit_item(item);
        }
    }
}

pub fn walk_generics<'ast, V: Visitor<'ast>>(v: &mut V, g: &'ast Generics) {
    for p in &g.params {
        v.visit_generic_param(p);
    }
    for w in &g.where_clause {
        v.visit_where_predicate(w);
    }
}

pub fn walk_generic_param<'ast, V: Visitor<'ast>>(v: &mut V, p: &'ast GenericParam) {
    v.visit_ident(&p.ident);
    match &p.kind {
        GenericParamKind::Type { bounds, default } => {
            bounds.iter().for_each(|b| v.visit_trait_bound(b));
            if let Some(d) = default {
                v.visit_ty(d);
            }
        }
        GenericParamKind::Const { ty, default } => {
            v.visit_ty(ty);
            if let Some(d) = default {
                v.visit_expr(d);
            }
        }
    }
}

pub fn walk_where_predicate<'ast, V: Visitor<'ast>>(v: &mut V, p: &'ast WherePredicate) {
    v.visit_ty(&p.bounded_ty);
    for b in &p.bounds {
        v.visit_trait_bound(b);
    }
}

pub fn walk_trait_bound<'ast, V: Visitor<'ast>>(v: &mut V, b: &'ast TraitBound) {
    v.visit_path(&b.path);
}

pub fn walk_path<'ast, V: Visitor<'ast>>(v: &mut V, p: &'ast Path) {
    for seg in &p.segments {
        v.visit_path_segment(seg);
    }
}

pub fn walk_path_segment<'ast, V: Visitor<'ast>>(v: &mut V, s: &'ast PathSegment) {
    v.visit_ident(&s.ident);
    if let Some(args) = &s.args {
        v.visit_generic_args(args);
    }
}

pub fn walk_generic_args<'ast, V: Visitor<'ast>>(v: &mut V, a: &'ast GenericArgs) {
    for arg in &a.args {
        match arg {
            GenericArg::Type(t) => v.visit_ty(t),
            GenericArg::Const(e) => v.visit_expr(e),
        }
    }
}

pub fn walk_ty<'ast, V: Visitor<'ast>>(v: &mut V, t: &'ast Type) {
    match &t.kind {
        TypeKind::Path(p) => v.visit_path(p),
        TypeKind::Ref { inner, .. } | TypeKind::Ptr { inner, .. } => v.visit_ty(inner),
        TypeKind::Tuple(tys) => tys.iter().for_each(|t| v.visit_ty(t)),
        TypeKind::Array { elem, len } => {
            v.visit_ty(elem);
            v.visit_expr(len);
        }
        TypeKind::Slice(elem) => v.visit_ty(elem),
        TypeKind::Fn { params, ret } => {
            params.iter().for_each(|t| v.visit_ty(t));
            if let Some(r) = ret {
                v.visit_ty(r);
            }
        }
        TypeKind::Never | TypeKind::Infer => {}
    }
}

pub fn walk_pat<'ast, V: Visitor<'ast>>(v: &mut V, p: &'ast Pat) {
    match &p.kind {
        PatKind::Wildcard => {}
        PatKind::Binding {
            ident, subpattern, ..
        } => {
            v.visit_ident(ident);
            if let Some(sub) = subpattern {
                v.visit_pat(sub);
            }
        }
        PatKind::Lit(e) => v.visit_expr(e),
        PatKind::Range { start, end, .. } => {
            if let Some(s) = start {
                v.visit_expr(s);
            }
            if let Some(e) = end {
                v.visit_expr(e);
            }
        }
        PatKind::Tuple(pats) | PatKind::Or(pats) => pats.iter().for_each(|p| v.visit_pat(p)),
        PatKind::Path(path) => v.visit_path(path),
        PatKind::TupleStruct { path, elems } => {
            v.visit_path(path);
            elems.iter().for_each(|p| v.visit_pat(p));
        }
        PatKind::Struct { path, fields, .. } => {
            v.visit_path(path);
            for f in fields {
                v.visit_ident(&f.ident);
                v.visit_pat(&f.pat);
            }
        }
        PatKind::Ref { inner, .. } => v.visit_pat(inner),
    }
}

pub fn walk_expr<'ast, V: Visitor<'ast>>(v: &mut V, e: &'ast Expr) {
    match &e.kind {
        ExprKind::Lit(l) => v.visit_literal(l),
        ExprKind::Path(p) => v.visit_path(p),
        ExprKind::Paren(inner) | ExprKind::Deref(inner) => v.visit_expr(inner),
        ExprKind::Unary { operand, .. } | ExprKind::AddrOf { operand, .. } => v.visit_expr(operand),
        ExprKind::Binary { lhs, rhs, .. }
        | ExprKind::Assign { lhs, rhs }
        | ExprKind::AssignOp { lhs, rhs, .. } => {
            v.visit_expr(lhs);
            v.visit_expr(rhs);
        }
        ExprKind::Cast { expr, ty } => {
            v.visit_expr(expr);
            v.visit_ty(ty);
        }
        ExprKind::Call { callee, args } => {
            v.visit_expr(callee);
            args.iter().for_each(|a| v.visit_expr(a));
        }
        ExprKind::MethodCall {
            receiver,
            segment,
            args,
        } => {
            v.visit_expr(receiver);
            v.visit_path_segment(segment);
            args.iter().for_each(|a| v.visit_expr(a));
        }
        ExprKind::Field { base, field } => {
            v.visit_expr(base);
            v.visit_ident(field);
        }
        ExprKind::TupleField { base, .. } => v.visit_expr(base),
        ExprKind::Index { base, index } => {
            v.visit_expr(base);
            v.visit_expr(index);
        }
        ExprKind::Range { start, end, .. } => {
            if let Some(s) = start {
                v.visit_expr(s);
            }
            if let Some(e) = end {
                v.visit_expr(e);
            }
        }
        ExprKind::Tuple(es) | ExprKind::Array(es) => es.iter().for_each(|e| v.visit_expr(e)),
        ExprKind::ArrayRepeat { elem, len } => {
            v.visit_expr(elem);
            v.visit_expr(len);
        }
        ExprKind::StructLit { path, fields } => {
            v.visit_path(path);
            for f in fields {
                v.visit_ident(&f.ident);
                v.visit_expr(&f.value);
            }
        }
        ExprKind::Block(b) | ExprKind::Unsafe(b) | ExprKind::Loop { body: b } => v.visit_block(b),
        ExprKind::If {
            cond,
            then_branch,
            else_branch,
        } => {
            v.visit_expr(cond);
            v.visit_block(then_branch);
            if let Some(e) = else_branch {
                v.visit_expr(e);
            }
        }
        ExprKind::Match { scrutinee, arms } => {
            v.visit_expr(scrutinee);
            for arm in arms {
                v.visit_pat(&arm.pat);
                if let Some(g) = &arm.guard {
                    v.visit_expr(g);
                }
                v.visit_expr(&arm.body);
            }
        }
        ExprKind::While { cond, body } => {
            v.visit_expr(cond);
            v.visit_block(body);
        }
        ExprKind::For { header, body } => {
            match header {
                ForHeader::Anonymous { iterable } => v.visit_expr(iterable),
                ForHeader::Named {
                    binding,
                    iterable,
                    step,
                } => {
                    v.visit_ident(binding);
                    v.visit_expr(iterable);
                    if let Some(step) = step {
                        v.visit_ident(&step.target);
                        v.visit_expr(&step.amount);
                    }
                }
            }
            v.visit_block(body);
        }
        ExprKind::Break(value) | ExprKind::Return(value) => {
            if let Some(e) = value {
                v.visit_expr(e);
            }
        }
        ExprKind::Continue => {}
    }
}

pub fn walk_block<'ast, V: Visitor<'ast>>(v: &mut V, b: &'ast Block) {
    for stmt in &b.stmts {
        v.visit_stmt(stmt);
    }
    if let Some(tail) = &b.tail {
        v.visit_expr(tail);
    }
}

pub fn walk_stmt<'ast, V: Visitor<'ast>>(v: &mut V, s: &'ast Stmt) {
    match &s.kind {
        StmtKind::Let(l) => {
            v.visit_pat(&l.pat);
            if let Some(ty) = &l.ty {
                v.visit_ty(ty);
            }
            if let Some(init) = &l.init {
                v.visit_expr(init);
            }
        }
        StmtKind::Item(i) => v.visit_item(i),
        StmtKind::Expr(e) | StmtKind::Semi(e) => v.visit_expr(e),
        StmtKind::Empty => {}
    }
}

#[cfg(test)]
mod tests {
    use gradi_core::span::Span;

    use super::*;
    use crate::{expr::BinaryOp, literal::LitKind, node_id::NodeIdGen};

    #[derive(Default)]
    struct ExprCounter(usize);

    impl<'ast> Visitor<'ast> for ExprCounter {
        fn visit_expr(&mut self, e: &'ast Expr) {
            self.0 += 1;
            walk_expr(self, e);
        }
    }

    fn int(ids: &mut NodeIdGen, value: u128) -> Expr {
        let span = Span::default();
        let lit = Literal {
            kind: LitKind::Int {
                value,
                suffix: None,
            },
            span,
        };
        Expr::new(ids.next_id(), ExprKind::Lit(lit), span)
    }

    #[test]
    fn counts_every_expression() {
        let mut ids = NodeIdGen::new();
        let lhs = int(&mut ids, 1);
        let rhs = int(&mut ids, 2);
        let sum = Expr::new(
            ids.next_id(),
            ExprKind::Binary {
                op: BinaryOp::Add,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            Span::default(),
        );

        let mut counter = ExprCounter::default();
        counter.visit_expr(&sum);
        assert_eq!(counter.0, 3);
    }

    #[test]
    fn operator_precedence_is_ordered() {
        assert!(BinaryOp::Mul.precedence() > BinaryOp::Add.precedence());
        assert!(BinaryOp::Add.precedence() > BinaryOp::Lt.precedence());
        assert!(BinaryOp::And.precedence() > BinaryOp::Or.precedence());
    }
}
