use lexor_reducer::{EngineView, NF, ReductionStrat};
use serde::{Deserialize, Serialize};

pub type SourceID = usize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerRequest {
    pub source_id: SourceID,
    pub code: String,
    pub wants_steps: bool,
    pub wants_graph: bool,
}

// TODO: Figure out graph nodes data type
pub type NodeData = ();

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerResponse {
    pub source_id: usize,
    pub steps: Option<Vec<String>>,
    pub graph_nodes: Option<Vec<NodeData>>,
}

pub fn process_job(req_json: &str) -> String {
    let req: WorkerRequest = serde_json::from_str(req_json).expect("Invalid JSON recieved");

    if req.code.is_empty() {
        let response = WorkerResponse {
            source_id: req.source_id,
            steps: req.wants_steps.then_some(vec![]),
            graph_nodes: req.wants_graph.then_some(vec![]),
        };

        return serde_json::to_string(&response).expect("Response serialization error");
    }

    let mut steps: Vec<String> = vec![req.code.clone()];
    let mut graph: Vec<NodeData> = vec![];

    NF::compute_with(&req.code, |view: EngineView| {
        if req.wants_steps {
            steps.push(format!("{view}"));
        }

        if req.wants_graph {
            graph.push(()); // TODO: implement
        }
    })
    .expect("Reduction failed");

    let response = WorkerResponse {
        source_id: req.source_id,
        steps: req.wants_steps.then_some(steps),
        graph_nodes: req.wants_graph.then_some(graph),
    };

    serde_json::to_string(&response).expect("Response serialization error")
}
