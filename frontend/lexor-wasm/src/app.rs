#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    messages::{AppMessage, SourceType},
    settings::Settings,
    state::AppState,
    tab_viewer::LexorTabViewer,
    tabs::AppTabs,
    worker_bridge::WorkerBridge,
};
use egui::{CentralPanel, Frame, TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use lexor_api::{SourceID, WorkerRequest, visual::RenderToken};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LexorApp {
    state: AppState,
    settings: Settings,
    tree: DockState<AppTabs>,
    #[serde(skip)]
    worker: Option<WorkerBridge>,
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
        let worker = None;

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
        if self.worker.is_none() {
            let bridge = WorkerBridge::new(self.state.messages.clone(), ctx.clone());
            self.worker = Some(bridge);
        }
        self.handle_debouncers(ctx);
        self.process_message_queue();
        self.set_theme(ctx);
        self.draw_canvas(ctx);
    }
}

impl LexorApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage
            && let Some(state) = eframe::get_value(storage, eframe::APP_KEY)
        {
            state
        } else {
            Self::default()
        }
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

    fn process_message_queue(&mut self) {
        self.state
            .messages
            .take()
            .into_iter()
            .for_each(|msg| self.apply_message(msg));
    }

    fn apply_message(&mut self, msg: AppMessage) {
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

                let wants_steps = self.is_steps_open_for(source_id);
                let wants_graph = self.is_graph_open_for(source_id);

                let request = WorkerRequest {
                    source_id,
                    code: input.clone(),
                    wants_steps,
                    wants_graph,
                };

                if let Some(bridge) = self.worker.as_ref() {
                    bridge.send_job(&request);
                }

                // Set loading screen while waiting
                // TODO: Refactor into something cleaner later
                if wants_steps {
                    self.state.reduction_steps.insert(
                        source_id,
                        vec![vec![RenderToken {
                            text: "Loading...".to_owned(),
                            style: lexor_api::visual::TokenStyle::Normal,
                            node_key: None,
                        }]],
                    );
                }
            }
            AppMessage::CloseSourceTab(source_id) => {
                self.tree.retain_tabs(|tab| match tab {
                    AppTabs::ReductionChain(out_id) => *out_id != source_id,
                    _ => true,
                });
                self.state.inputs.remove(&source_id);
                self.state.reduction_steps.remove(&source_id);
                self.state.last_assigned_key = *self.state.inputs.keys().max().unwrap_or(&0usize);
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

    fn is_steps_open_for(&self, source_id: SourceID) -> bool {
        self.tree.iter_all_tabs().any(
            |(_, tab)| matches!(tab, AppTabs::ReductionChain(inner_id) if source_id == *inner_id),
        )
    }

    fn is_graph_open_for(&self, source_id: SourceID) -> bool {
        self.tree.iter_all_tabs().any(
            |(_, tab)| matches!(tab, AppTabs::ReductionGraph(inner_id) if source_id == *inner_id),
        )
    }

    fn handle_debouncers(&mut self, ctx: &egui::Context) {
        let current_time = ctx.input(|i| i.time);
        let debounce_delay = 0.5;
        let mut to_recompile = Vec::new();

        self.state.last_edited_time.retain(|&id, &mut last_time| {
            if current_time - last_time > debounce_delay {
                to_recompile.push(id);
                false
            } else {
                true
            }
        });

        for id in to_recompile {
            self.state
                .messages
                .borrow_mut()
                .push(AppMessage::SendReductionJob(id));
        }
    }

    fn set_theme(&self, ctx: &egui::Context) {
        match self.state.style {
            // Catpuccin, Gruvbox, Compiler Explorer
        }
    }

    fn draw_canvas(&mut self, ctx: &egui::Context) {
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
