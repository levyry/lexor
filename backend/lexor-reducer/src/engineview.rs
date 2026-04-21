/*!
This module containts the [`EngineView`] struct, which is a view to the internal
reduction engine that is provided by callbacks.
*/

use slotmap::Key;

use crate::core::{
    engine::Engine,
    node::{Node, NodeComb, NodeKey},
};

use core::fmt::{self, Display};
use std::collections::HashSet;

pub type ArgIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeRole {
    Normal,
    RedexHead(NodeComb),
    RedexArg(NodeComb, ArgIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EngineGraphNodeKind {
    App,
    Comb(NodeComb),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EngineGraphNode {
    pub id: u64,
    pub kind: EngineGraphNodeKind,
    pub children: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct EngineView<'a>(&'a Engine);

impl<'a> EngineView<'a> {
    #[must_use]
    pub(crate) const fn from_engine(engine: &'a Engine) -> Self {
        Self(engine)
    }

    /// Returns the current depth of the spine (the "stack" of the machine).
    /// This gives an indication of how deep into the term the reducer is currently operating.
    #[must_use]
    pub const fn stack_depth(self) -> usize {
        self.0.spine.len()
    }

    pub fn traverse(&self, mut visitor: impl FnMut(&str, NodeRole, Option<NodeKey>)) {
        let spine = &self.0.spine;
        let arena = &self.0.arena;

        let mut redex_parent = None;
        let mut redex_args = Vec::new();
        let mut active_comb = None;

        if let Some(&head_key) = spine.last()
            && let Ok(Node::Comb(comb)) = arena.get(head_key)
            && spine.len() > comb.arity()
        {
            let parent_index = spine.len().saturating_sub(2);
            redex_parent = Some(
                *spine
                    .get(parent_index)
                    .expect("If conditions should guard against a panic here."),
            );
            active_comb = Some(*comb);

            for i in 1..=comb.arity() {
                let app_index = spine.len().saturating_sub(1).saturating_sub(i);
                let app_key = *spine.get(app_index).expect("Because we loop based on the combinator arity, the spine should be sufficiently filled to avoid this panic.");
                if let Ok(Node::App(_, rhs)) = arena.get(app_key) {
                    let mut target = *rhs;
                    while let Ok(Node::Indirection(t)) = arena.get(target) {
                        target = *t;
                    }
                    redex_args.push(target);
                }
            }
        }

        self.traverse_to_tokens(redex_parent, &redex_args, active_comb, &mut visitor);
    }

    fn traverse_to_tokens(
        self,
        redex_parent: Option<NodeKey>,
        redex_args: &[NodeKey],
        active_comb: Option<NodeComb>,
        visitor: &mut dyn FnMut(&str, NodeRole, Option<NodeKey>),
    ) {
        let initial_state = TraverseState {
            current_key: self.0.root,
            current_role: NodeRole::Normal,
            is_rhs_of_app: false,
            is_redex_head: false,
        };

        self.traverse_to_tokens_rec(
            initial_state,
            redex_parent,
            redex_args,
            active_comb,
            visitor,
        );
    }

    fn traverse_to_tokens_rec(
        self,
        mut state: TraverseState,
        redex_parent: Option<NodeKey>,
        redex_args: &[NodeKey],
        active_comb: Option<NodeComb>,
        visitor: &mut dyn FnMut(&str, NodeRole, Option<NodeKey>),
    ) {
        let arena = &self.0.arena;
        let mut current = state.current_key;

        while let Ok(Node::Indirection(target)) = arena.get(current) {
            current = *target;
        }

        if state.current_role == NodeRole::Normal
            && let Some(arg_idx) = redex_args.iter().position(|&k| k == current)
            && let Some(comb) = active_comb
        {
            state.current_role = NodeRole::RedexArg(comb, arg_idx);
        }

        match arena.get(current) {
            Ok(Node::App(lhs, rhs)) => {
                if state.is_rhs_of_app {
                    visitor("(", state.current_role, Some(current));
                }

                let is_lhs_head = Some(current) == redex_parent;

                self.traverse_to_tokens_rec(
                    TraverseState {
                        current_key: *lhs,
                        current_role: state.current_role,
                        is_rhs_of_app: false,
                        is_redex_head: is_lhs_head,
                    },
                    redex_parent,
                    redex_args,
                    active_comb,
                    visitor,
                );

                // visitor(" ", current_role, Some(current));

                self.traverse_to_tokens_rec(
                    TraverseState {
                        current_key: *rhs,
                        current_role: state.current_role,
                        is_rhs_of_app: true,
                        is_redex_head: false,
                    },
                    redex_parent,
                    redex_args,
                    active_comb,
                    visitor,
                );

                if state.is_rhs_of_app {
                    visitor(")", state.current_role, Some(current));
                }
            }
            Ok(Node::Comb(x)) => {
                let role = if state.is_redex_head {
                    active_comb.map_or(NodeRole::RedexHead(*x), NodeRole::RedexHead)
                } else {
                    state.current_role
                };
                visitor(x.to_string().as_str(), role, Some(current));
            }
            Ok(Node::Indirection(_)) => unreachable!(),
            Err(err) => visitor(err.to_string().as_str(), NodeRole::Normal, Some(current)),
        }
    }

    /// Extracts the currently reachable graph using backend types.
    #[must_use]
    pub fn extract_graph(&self) -> Vec<EngineGraphNode> {
        let mut graph = Vec::new();
        let mut visited = HashSet::new();
        let mut work_stack = vec![self.0.root];

        while let Some(current) = work_stack.pop() {
            let mut resolved_key = current;
            while let Ok(Node::Indirection(target)) = self.0.arena.get(resolved_key) {
                resolved_key = *target;
            }

            if !visited.insert(resolved_key) {
                continue;
            }

            if let Ok(node) = self.0.arena.get(resolved_key) {
                let id = resolved_key.data().as_ffi();

                match node {
                    Node::Comb(comb) => {
                        graph.push(EngineGraphNode {
                            id,
                            kind: EngineGraphNodeKind::Comb(*comb),
                            children: vec![],
                        });
                    }
                    Node::App(lhs, rhs) => {
                        let mut resolved_lhs = *lhs;
                        while let Ok(Node::Indirection(t)) = self.0.arena.get(resolved_lhs) {
                            resolved_lhs = *t;
                        }

                        let mut resolved_rhs = *rhs;
                        while let Ok(Node::Indirection(t)) = self.0.arena.get(resolved_rhs) {
                            resolved_rhs = *t;
                        }

                        let id_lhs = resolved_lhs.data().as_ffi();
                        let id_rhs = resolved_rhs.data().as_ffi();

                        graph.push(EngineGraphNode {
                            id,
                            kind: EngineGraphNodeKind::App,
                            children: vec![id_lhs, id_rhs],
                        });

                        work_stack.push(*lhs);
                        work_stack.push(*rhs);
                    }
                    Node::Indirection(_) => unreachable!(),
                }
            }
        }

        graph
    }
}

// TODO: Look into handling the two errors cases (resolved indirections and
//       missing nodes from arena) instead of just writing the error to the
//       result.
impl Display for EngineView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        self.traverse(|text, _role, _key| {
            output.push_str(text);
        });
        write!(f, "{output}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TraverseState {
    current_key: NodeKey,
    current_role: NodeRole,
    is_rhs_of_app: bool,
    is_redex_head: bool,
}
