use lexor_api::{SourceID, WorkerResponse};
use serde::{Deserialize, Serialize};

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
    SendReductionJob(SourceID),
    SetGraphStep(SourceID, usize),
    WorkerJobCompleted(WorkerResponse),
    CloseSourceTab(SourceID),
}
