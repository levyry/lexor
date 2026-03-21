use lexor_api::SourceID;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppTabs {
    Welcome,
    SkiSource(SourceID),
    ReductionChain(SourceID),
    ReductionGraph(SourceID),
}
