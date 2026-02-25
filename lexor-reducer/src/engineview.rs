/*!
This module containts the [`EngineView`] struct, which is a view to the internal
reduction engine that is provided by callbacks.
*/

use crate::core::{
    arena::Arena,
    engine::Engine,
    node::{Node, NodeKey},
};
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub struct EngineView<'a>(&'a Engine);

// TODO: Expand later so the highlighted segments can be extracted from
//       this in addition to the String representation.
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
}

impl Display for EngineView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node(
            arena: &Arena,
            key: NodeKey,
            f: &mut fmt::Formatter<'_>,
            is_rhs_of_app: bool,
        ) -> fmt::Result {
            let mut current = key;
            while let Some(Node::Indirection(target)) = arena.get(current) {
                current = *target;
            }

            match arena.get(current) {
                Some(Node::App(lhs, rhs)) => {
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
                Some(Node::Comb(x)) => write!(f, "{x}"),
                Some(Node::Indirection(_)) => unreachable!("Indirections resolved already"),
                None => unreachable!("Node not found in arena"),
            }
        }

        print_node(&self.0.arena, self.0.root, f, false)
    }
}
