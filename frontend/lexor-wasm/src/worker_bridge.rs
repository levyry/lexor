use std::{cell::RefCell, rc::Rc};

use lexor_api::request::WorkerRequest;
use lexor_api::response::WorkerResponse;
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
            if let Some(json_str) = event.data().as_string() {
                match serde_json::from_str::<WorkerResponse>(&json_str) {
                    Ok(response) => {
                        queue
                            .borrow_mut()
                            .push(AppMessage::WorkerJobCompleted(response));

                        ctx.request_repaint();
                    }
                    Err(err) => {
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::error_1(
                            &format!(
                                "DESERIALIZATION PANIC!\nError: {}\nJSON Received: {}",
                                err, json_str
                            )
                            .into(),
                        );
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!("{err}")
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget(); // forgor

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
