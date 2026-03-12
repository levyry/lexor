use serde::{Deserialize, Serialize};

pub type SourceID = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceType {
    Ski,
    Lambda,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppMessage {
    RequestNewSource(SourceType),
    RequestChainOutput(SourceID),
    RequestGraphOutput(SourceID),
    RunReduction(SourceID),
    CloseSourceTab(SourceID),
}
