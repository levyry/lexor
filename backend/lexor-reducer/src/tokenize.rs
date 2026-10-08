use lexor_core::literal::Literal;

use crate::core::{
    arena::Arena,
    node::{Node, NodeComb, NodeKey},
    visitor::{self, Visitor},
};

pub type ArgIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EngineToken {
    pub text: String,
    pub role: NodeRole,
    pub key: Option<NodeKey>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeRole {
    Normal,
    RedexHead(NodeComb),
    RedexArg(NodeComb, ArgIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TraverseState {
    pub current_role: NodeRole,
    pub is_rhs_of_app: bool,
    pub is_redex_head: bool,
}

pub(crate) struct TokenizeVisitor<'a> {
    pub arena: &'a Arena,
    pub state: TraverseState,
    pub redex_parent: Option<NodeKey>,
    pub redex_args: &'a [NodeKey],
    pub active_comb: Option<NodeComb>,
    pub tokens: &'a mut Vec<EngineToken>,
    pub root: NodeKey,
}

impl<'a> TokenizeVisitor<'_> {
    fn push(&mut self, text: &str, role: NodeRole, key: Option<NodeKey>) {
        self.tokens.push(EngineToken {
            text: text.to_owned(),
            role,
            key,
        });
    }
}

impl<'a> Visitor<'a> for TokenizeVisitor<'a> {
    fn arena(&self) -> &'a Arena {
        self.arena
    }

    fn visit_node(&mut self, key: NodeKey) {
        let old_state = self.state;

        if self.state.current_role == NodeRole::Normal
            && let Some(arg_idx) = self.redex_args.iter().position(|&k| k == key)
            && let Some(comb) = self.active_comb
        {
            self.state.current_role = NodeRole::RedexArg(comb, arg_idx.saturating_add(1));
        }

        if let Err(err) = self.arena().get(key) {
            self.push(err.to_string().as_str(), NodeRole::Normal, Some(key));
        } else {
            visitor::walk_node(self, key);
        }

        self.state = old_state;
    }

    fn visit_app(&mut self, key: NodeKey, lhs: NodeKey, rhs: NodeKey) {
        if let Ok(Node::App(op_key, left_key)) = self.arena().get(lhs)
            && let Ok(Node::Prim(Literal::Op(op))) = self.arena().get(*op_key)
        {
            let need_parens = key != self.root;

            if need_parens {
                self.push("(", self.state.current_role, Some(key));
            }

            let old_state = self.state;
            self.state.is_rhs_of_app = false;
            self.visit_node(*left_key);

            self.push(" ", self.state.current_role, None);
            self.push(
                op.to_string().as_str(),
                self.state.current_role,
                Some(*op_key),
            );
            self.push(" ", self.state.current_role, None);

            self.visit_node(rhs);

            self.state = old_state;

            if need_parens {
                self.push(")", self.state.current_role, Some(key));
            }
        } else {
            if self.state.is_rhs_of_app {
                self.push("(", self.state.current_role, Some(key));
            }

            let is_lhs_head = Some(key) == self.redex_parent;

            let old_state = self.state;
            self.state.is_rhs_of_app = false;
            self.state.is_redex_head = is_lhs_head;
            self.visit_node(lhs);

            self.state = old_state;
            self.state.is_rhs_of_app = true;
            self.visit_node(rhs);

            self.state = old_state;
            if self.state.is_rhs_of_app {
                self.push(")", self.state.current_role, Some(key));
            }
        }
    }

    fn visit_comb(&mut self, key: NodeKey, comb: NodeComb) {
        let role = if self.state.is_redex_head {
            self.active_comb
                .map_or(NodeRole::RedexHead(comb), NodeRole::RedexHead)
        } else {
            self.state.current_role
        };
        self.push(comb.to_string().as_str(), role, Some(key));
    }

    fn visit_prim(&mut self, key: NodeKey, prim: &'a Literal) {
        let role = self.state.current_role;
        self.push(prim.to_string().as_str(), role, Some(key));
    }
}
