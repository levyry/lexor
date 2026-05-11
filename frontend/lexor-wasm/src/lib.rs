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
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn process_worker_job(json: String) -> String {
    lexor_api::process_job(&json)
}
