use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[repr(transparent)]
pub struct SourceID(pub usize);

impl SourceID {
    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
    }
}

impl From<usize> for SourceID {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl From<SourceID> for usize {
    fn from(id: SourceID) -> Self {
        id.0
    }
}

impl AsRef<usize> for SourceID {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl fmt::Display for SourceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
