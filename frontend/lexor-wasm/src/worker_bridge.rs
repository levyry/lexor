use std::{cell::RefCell, rc::Rc};

use lexor_api::{WorkerRequest, WorkerResponse};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::messages::AppMessage;

pub struct WorkerBridge {
    worker: web_sys::Worker,
}

impl WorkerBridge {
    pub fn new(queue: Rc<RefCell<Vec<AppMessage>>>) -> Self {
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);

        let worker = web_sys::Worker::new_with_options("worker.js", &options)
            .expect("Failed to create Web Worker");

        #[allow(clippy::as_conversions)]
        let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            if let Some(json_str) = event.data().as_string() {
                let response: WorkerResponse = serde_json::from_str(&json_str)
                    .expect("Failed to deserialize response in closure");
                queue
                    .borrow_mut()
                    .push(AppMessage::WorkerJobCompleted(response));
            }
        }) as Box<dyn FnMut(_)>);

        worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget();

        Self { worker }
    }

    pub fn send_job(&self, request: &WorkerRequest) {
        let json = serde_json::to_string(&request)
            .expect("Failed to serialize into JSON when sending request");
        self.worker
            .post_message(&json.into())
            .expect("Worker returned Err");
    }
}
