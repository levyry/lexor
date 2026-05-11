use lexor_api::{SourceID, response::ReductionResponse, source_id::SourceKind, visual::VisualComb};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppMessage {
    RequestNewSource(SourceKind),
    RequestChainOutput(SourceID),
    RequestGraphOutput(SourceID),
    SendSkiReductionJob(SourceID),
    SendLambdaReductionJob(SourceID),
    SetGraphStep(SourceID, usize),
    ReductionJobCompleted(ReductionResponse),
    CloseSourceTab(SourceID),
    AddLambdaInput(SourceID, VisualComb, f64),
    ConvertSkiToLambda(SourceID),
    ConversionCompleted(SourceID, String),
}
