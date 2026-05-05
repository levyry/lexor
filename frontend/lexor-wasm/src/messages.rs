use lexor_api::{SourceID, WorkerResponse};
use serde::{Deserialize, Serialize};

use crate::source::SourceKind;


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppMessage {
    RequestNewSource(SourceKind),
    RequestChainOutput(SourceID),
    RequestGraphOutput(SourceID),
    SendReductionJob(SourceID),
    SetGraphStep(SourceID, usize),
    WorkerJobCompleted(WorkerResponse),
    CloseSourceTab(SourceID),
}
