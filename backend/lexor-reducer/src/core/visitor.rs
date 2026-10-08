use crate::core::{
    arena::Arena,
    node::{Node, NodeComb, NodeKey},
};
use lexor_core::literal::Literal;

pub trait Visitor<'a> {
    fn arena(&self) -> &'a Arena;

    fn visit_node(&mut self, key: NodeKey) {
        walk_node(self, key);
    }

    fn visit_app(&mut self, key: NodeKey, lhs: NodeKey, rhs: NodeKey) {
        walk_app(self, key, lhs, rhs);
    }

    fn visit_comb(&mut self, _key: NodeKey, _comb: NodeComb) {}

    fn visit_prim(&mut self, _key: NodeKey, _prim: &'a Literal) {}
}

pub fn walk_node<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, key: NodeKey) {
    match visitor.arena().get(key) {
        Ok(Node::App(lhs, rhs)) => visitor.visit_app(key, *lhs, *rhs),
        Ok(Node::Comb(comb)) => visitor.visit_comb(key, *comb),
        Ok(Node::Prim(prim)) => visitor.visit_prim(key, prim),
        Ok(Node::Indirection(_)) => unreachable!("arena.get() hides indirections"),
        Err(_) => unreachable!("arena.get() key not found during walk"),
    }
}

pub fn walk_app<'a, V: Visitor<'a> + ?Sized>(
    visitor: &mut V,
    _key: NodeKey,
    lhs: NodeKey,
    rhs: NodeKey,
) {
    visitor.visit_node(lhs);
    visitor.visit_node(rhs);
}
