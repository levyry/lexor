pub mod app;
pub mod graph;
pub mod messages;
pub mod node_style;
pub mod settings;
pub mod source;
pub mod state;
pub mod tab_viewer;
pub mod worker_bridge;

#[cfg(target_arch = "wasm32")]
use lexor_api::{WorkerResult, WorkerTask, convert_ski_to_lambda_string, reduce_expression};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn process_worker_job(json: &str) -> String {
    let task: WorkerTask = serde_json::from_str(json).expect("Invalid WorkerTask JSON recieved");

    let payload = match task {
        WorkerTask::Reduction(req) => {
            let result = reduce_expression(&req);

            WorkerResult::Reduction(result)
        }
        WorkerTask::Conversion { source_id, input } => {
            let result = match convert_ski_to_lambda_string(&input) {
                Ok(res) => res,
                Err(err) => err,
            };

            WorkerResult::Conversion { source_id, result }
        }
    };

    serde_json::to_string(&payload).expect("WorkerResult serialization failed")
}
