/*!
This module containts the [`EngineView`] struct, which is a view to the internal
reduction engine that is provided by callbacks.
*/

use crate::{
    core::{arena::Arena, engine::Engine},
    graphred::RealArena,
};
use core::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct EngineView<'a>(&'a Engine<RealArena>);

impl Display for EngineView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: Expand later so the highlighted segments can be extracted from
//       this in addition to the String representation.
impl<'a> EngineView<'a> {
    #[must_use]
    pub(crate) const fn from_engine(engine: &'a Engine<RealArena>) -> Self {
        Self(engine)
    }

    /// Returns the total number of nodes currently allocated in the arena.
    #[must_use]
    pub fn node_count(self) -> usize {
        self.0.arena.size()
    }

    /// Returns the current depth of the spine (the "stack" of the machine).
    /// This gives an indication of how deep into the term the reducer is currently operating.
    #[must_use]
    pub const fn stack_depth(self) -> usize {
        self.0.spine.len()
    }
}
