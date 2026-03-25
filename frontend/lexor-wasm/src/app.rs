#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    messages::{AppMessage, SourceType},
    settings::Settings,
    state::AppState,
    tab_viewer::LexorTabViewer,
    tabs::AppTabs,
};
use egui::{CentralPanel, Frame, TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use lexor_api::{SourceID, WorkerRequest, WorkerResponse};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::Closure};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LexorApp {
    state: AppState,
    settings: Settings,
    tree: DockState<AppTabs>,
    #[serde(skip)]
    worker: web_sys::Worker,
}

impl Default for LexorApp {
    fn default() -> Self {
        let mut state = AppState::default();
        let ski_tab = state.new_ski_source();
        let AppTabs::SkiSource(id) = ski_tab else {
            unreachable!()
        };
        let reduction_tab = state.new_reduction_output(id);

        let mut tree = DockState::new(vec![AppTabs::Welcome]);

        let [_welcome_node, ski_node] =
            tree.main_surface_mut()
                .split_above(NodeIndex::root(), 0.3, vec![ski_tab]);

        let [_, _] = tree
            .main_surface_mut()
            .split_right(ski_node, 0.5, vec![reduction_tab]);

        let message_queue = Rc::new(RefCell::new(Vec::new()));
        let worker = web_sys::Worker::new("worker.js").expect("Failed to create Web Worker");

        let callback_queue = Rc::clone(&message_queue);

        #[allow(clippy::as_conversions)]
        let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            if let Some(json_str) = event.data().as_string() {
                let response: WorkerResponse = serde_json::from_str(&json_str)
                    .expect("Failed to deserialize response in closure");
                callback_queue
                    .borrow_mut()
                    .push(AppMessage::WorkerJobCompleted(response));
            }
        }) as Box<dyn FnMut(_)>);

        worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget(); // Keep callback alive

        state.messages = message_queue;

        Self {
            state,
            tree,
            worker,
            settings: Settings::default(),
        }
    }
}

impl eframe::App for LexorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_msg_queue(ctx);

        // Draw scene
        TopBottomPanel::top("egui_dock::MenuBar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| self.add_menubar(ui))
        });

        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                let style = self
                    .state
                    .style
                    .get_or_insert_with(|| Style::from_egui(ui.style()))
                    .clone();

                let mut tab_viewer = LexorTabViewer {
                    state: &mut self.state,
                };

                // Display view
                DockArea::new(&mut self.tree)
                    .style(style)
                    .show_inside(ui, &mut tab_viewer);
            });
    }
}

impl LexorApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: LexorApp = if let Some(storage) = cc.storage
            && let Some(state) = eframe::get_value(storage, eframe::APP_KEY)
        {
            state
        } else {
            Self::default()
        };

        let message_queue = Rc::new(RefCell::new(Vec::new()));

        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);

        let worker = web_sys::Worker::new_with_options("worker.js", &options)
            .expect("Failed to create Web Worker");

        let callback_queue = Rc::clone(&message_queue);

        #[allow(clippy::as_conversions)]
        let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            if let Some(json_str) = event.data().as_string() {
                let response: WorkerResponse = serde_json::from_str(&json_str)
                    .expect("Failed to deserialize response in closure");
                callback_queue
                    .borrow_mut()
                    .push(AppMessage::WorkerJobCompleted(response));
            }
        }) as Box<dyn FnMut(_)>);

        worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget();

        // 3. Connect them back to the app!
        app.worker = worker;
        app.state.messages = message_queue;

        app
    }

    fn add_menubar(&self, ui: &mut Ui) {
        ui.menu_button("Add", |ui| {
            if ui.button("SKI source").clicked() {
                self.state
                    .messages
                    .borrow_mut()
                    .push(AppMessage::RequestNewSource(SourceType::Ski));
                ui.close();
            }
        });
    }

    fn spawn_tab(&mut self, tab: AppTabs) {
        // Duplicate check
        if self.tree.find_tab(&tab).is_some() {
            return;
        }

        if self.tree.iter_all_tabs().count() == 0 {
            self.tree = DockState::new(vec![tab]);
            return;
        }

        let surface = self.tree.main_surface_mut();

        if let Some(target_node) = surface.focused_leaf() {
            surface.split_right(target_node, 0.5, vec![tab]);
        } else {
            surface.push_to_first_leaf(tab);
        }
    }

    fn process_msg_queue(&mut self, ctx: &egui::Context) {
        // Debouncer logic
        let current_time = ctx.input(|i| i.time);
        let debounce_delay = 0.5;

        // Find all tabs that need to be recompiled and push the
        // message to run the reduction on them
        let recompiled_tabs: Vec<SourceID> = self
            .state
            .last_edited_time
            .iter()
            .filter(|&(_id, last_time)| current_time - last_time > debounce_delay)
            .map(|(&id, _)| id)
            .collect();

        for id in recompiled_tabs {
            self.state.last_edited_time.remove(&id);
            if self.state.inputs.contains_key(&id) {
                self.state
                    .messages
                    .borrow_mut()
                    .push(AppMessage::SendReductionJob(id));
            }
        }

        // Handle the message queue
        let pending = self.state.messages.take();

        for msg in pending {
            match msg {
                AppMessage::RequestNewSource(SourceType::Ski) => {
                    let tab = self.state.new_ski_source();
                    self.spawn_tab(tab);
                }
                AppMessage::RequestNewSource(SourceType::Lambda) => {
                    todo!()
                }
                AppMessage::RequestChainOutput(source_id) => {
                    let tab = self.state.new_reduction_output(source_id);
                    self.spawn_tab(tab);
                }
                AppMessage::SendReductionJob(source_id) => {
                    let input = self
                        .state
                        .inputs
                        .get(&source_id)
                        .expect("SourceID not found while trying to run reduction");

                    let wants_steps = self.tree.iter_all_tabs().any(|tab| {
                        if let AppTabs::ReductionChain(inner_id) = tab.1
                            && source_id == *inner_id
                        {
                            true
                        } else {
                            false
                        }
                    });

                    let wants_graph = self.tree.iter_all_tabs().any(|tab| {
                        if let AppTabs::ReductionGraph(inner_id) = tab.1
                            && source_id == *inner_id
                        {
                            true
                        } else {
                            false
                        }
                    });

                    let request = WorkerRequest {
                        source_id,
                        code: input.clone(),
                        wants_steps,
                        wants_graph,
                    };

                    let json = serde_json::to_string(&request)
                        .expect("Failed to serialize into JSON when sending request");
                    self.worker
                        .post_message(&json.into())
                        .expect("Worker returned Err");

                    // Set loading screen while waiting
                    if wants_steps {
                        self.state
                            .reduction_steps
                            .insert(source_id, vec!["Loading...".to_owned()]);
                    }
                }
                AppMessage::CloseSourceTab(source_id) => {
                    self.tree.retain_tabs(|tab| match tab {
                        AppTabs::ReductionChain(out_id) => *out_id != source_id,
                        _ => true,
                    });
                    self.state.inputs.remove(&source_id);
                    self.state.reduction_steps.remove(&source_id);
                    self.state.last_assigned_key =
                        *self.state.inputs.keys().max().unwrap_or(&0usize);
                }
                AppMessage::RequestGraphOutput(_source_id) => todo!(),
                AppMessage::WorkerJobCompleted(worker_response) => {
                    if let Some(steps) = worker_response.steps {
                        self.state
                            .reduction_steps
                            .insert(worker_response.source_id, steps);
                    }

                    if let Some(graph) = worker_response.graph_nodes {
                        self.state
                            .reduction_graph
                            .insert(worker_response.source_id, graph);
                    }
                }
            }
        }
    }
}
