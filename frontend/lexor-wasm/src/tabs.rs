use serde::{Deserialize, Serialize};

use crate::messages::SourceID;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppTabs {
    Welcome,
    SkiSource(SourceID),
    ReductionChain(SourceID),
    ReductionGraph(SourceID),
}
