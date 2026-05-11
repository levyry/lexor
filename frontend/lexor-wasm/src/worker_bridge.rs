use std::{cell::RefCell, rc::Rc};

use lexor_api::{SourceID, WorkerResult};
use lexor_api::{WorkerTask, request::ReductionRequest};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::messages::AppMessage;

#[derive(Debug, Clone)]
pub struct WorkerBridge {
    worker: web_sys::Worker,
}

impl WorkerBridge {
    pub fn new(queue: Rc<RefCell<Vec<AppMessage>>>, ctx: egui::Context) -> Self {
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);

        let worker = web_sys::Worker::new_with_options("worker.js", &options)
            .expect("Failed to create Web Worker");

        #[allow(clippy::as_conversions)]
        let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            if let Some(json_str) = event.data().as_string()
                && let Ok(worker_result) = serde_json::from_str::<WorkerResult>(&json_str)
            {
                match worker_result {
                    WorkerResult::Reduction(response) => {
                        queue
                            .borrow_mut()
                            .push(AppMessage::ReductionJobCompleted(response));
                    }
                    WorkerResult::Conversion { source_id, result } => {
                        queue
                            .borrow_mut()
                            .push(AppMessage::ConversionCompleted(source_id, result));
                    }
                }
                ctx.request_repaint();
            }
        }) as Box<dyn FnMut(_)>);

        worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget(); // forgor

        Self { worker }
    }

    pub fn send_reduction_job(&self, request: ReductionRequest) {
        let task = WorkerTask::Reduction(request);

        let json =
            serde_json::to_string(&task).expect("WorkerTask JSON serialization failed (reduction)");

        self.worker
            .post_message(&json.into())
            .expect("Failed at posting message");
    }

    pub fn send_conversion_job(&self, id: SourceID, input: &str) {
        let task = WorkerTask::Conversion {
            source_id: id,
            input: String::from(input),
        };

        let json = serde_json::to_string(&task)
            .expect("WorkerTask JSON serialization failed (conversion)");

        self.worker
            .post_message(&json.into())
            .expect("Failed at posting message");
    }
}
