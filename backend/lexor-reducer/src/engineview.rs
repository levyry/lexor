/*!
This module containts the [`EngineView`] struct, which is a view to the internal
reduction engine that is provided by callbacks.
*/

use crate::core::{
    arena::Arena,
    engine::Engine,
    node::{Node, NodeComb, NodeKey},
};

use core::fmt::{self, Display};

pub type ArgIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeRole {
    Normal,
    RedexHead(NodeComb),
    RedexArg(NodeComb, ArgIndex),
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

        self.traverse_to_tokens(
            self.0.root,
            false,
            NodeRole::Normal,
            false,
            redex_parent,
            &redex_args,
            active_comb,
            &mut visitor,
        );
    }

    // TODO: what the fuck
    fn traverse_to_tokens(
        self,
        key: NodeKey,
        is_rhs_of_app: bool,
        mut current_role: NodeRole,
        is_redex_head: bool,
        redex_parent: Option<NodeKey>,
        redex_args: &[NodeKey],
        active_comb: Option<NodeComb>,
        visitor: &mut dyn FnMut(&str, NodeRole, Option<NodeKey>),
    ) {
        let arena = &self.0.arena;
        let mut current = key;

        while let Ok(Node::Indirection(target)) = arena.get(current) {
            current = *target;
        }

        if current_role == NodeRole::Normal
            && let Some(arg_idx) = redex_args.iter().position(|&k| k == current)
            && let Some(comb) = active_comb
        {
            current_role = NodeRole::RedexArg(comb, arg_idx);
        }

        match arena.get(current) {
            Ok(Node::App(lhs, rhs)) => {
                if is_rhs_of_app {
                    visitor("(", current_role, Some(current));
                }

                let is_lhs_head = Some(current) == redex_parent;

                self.traverse_to_tokens(
                    *lhs,
                    false,
                    current_role,
                    is_lhs_head,
                    redex_parent,
                    redex_args,
                    active_comb,
                    &mut *visitor,
                );

                // visitor(" ", current_role, Some(current));

                self.traverse_to_tokens(
                    *rhs,
                    true,
                    current_role,
                    false,
                    redex_parent,
                    redex_args,
                    active_comb,
                    &mut *visitor,
                );

                if is_rhs_of_app {
                    visitor(")", current_role, Some(current));
                }
            }
            Ok(Node::Comb(x)) => {
                let role = if is_redex_head {
                    active_comb.map_or(NodeRole::RedexHead(*x), NodeRole::RedexHead)
                } else {
                    current_role
                };
                visitor(x.to_string().as_str(), role, Some(current));
            }
            Ok(Node::Indirection(_)) => unreachable!(),
            Err(err) => visitor(err.to_string().as_str(), NodeRole::Normal, Some(current)),
        }
    }
}

// TODO: Look into handling the two errors cases (resolved indirections and
//       missing nodes from arena) instead of just writing the error to the
//       result.
impl Display for EngineView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node(
            arena: &Arena,
            key: NodeKey,
            f: &mut fmt::Formatter<'_>,
            is_rhs_of_app: bool,
        ) -> fmt::Result {
            let mut current = key;
            while let Ok(Node::Indirection(target)) = arena.get(current) {
                current = *target;
            }

            match arena.get(current) {
                Ok(Node::App(lhs, rhs)) => {
                    if is_rhs_of_app {
                        write!(f, "(")?;
                    }

                    print_node(arena, *lhs, f, false)?;
                    // write!(f, " ")?;
                    print_node(arena, *rhs, f, true)?;

                    if is_rhs_of_app {
                        write!(f, ")")?;
                    }
                    Ok(())
                }
                Ok(Node::Comb(x)) => write!(f, "{x}"),
                Ok(Node::Indirection(_)) => write!(f, "Somehow found an indirection"),
                Err(err) => write!(f, "{err}"),
            }
        }

        print_node(&self.0.arena, self.0.root, f, false)
    }
}
