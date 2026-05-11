#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

use crate::{
    graph::build_egui_graph,
    messages::AppMessage,
    state::AppState,
    tab_viewer::{AppTabs, LexorTabViewer},
    worker_bridge::WorkerBridge,
};
use egui::{CentralPanel, Frame, TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use lexor_api::{
    ApiStrategy, SourceID,
    request::{WorkerRequest, WorkerRequestState},
    response::WorkerResponseState,
    source_id::SourceKind,
    visual::VisualComb,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LexorApp {
    state: AppState,
    tree: DockState<AppTabs>,
    #[serde(skip)]
    worker: Option<WorkerBridge>,
}

impl Default for LexorApp {
    fn default() -> Self {
        let mut state = AppState::default();
        let ski_tab = state.new_source(SourceKind::Ski);
        let id = ski_tab.get_id();
        let reduction_tab = AppTabs::ReductionChain(id);

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
        // self.set_theme(ctx);
        self.draw_canvas(ctx);
    }
}

impl LexorApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

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
                    .push_msg(AppMessage::RequestNewSource(SourceKind::Ski));
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
            AppMessage::RequestNewSource(kind) => {
                let tab = self.state.new_source(kind);
                self.spawn_tab(tab);
            }
            AppMessage::RequestChainOutput(source_id) => {
                self.spawn_tab(AppTabs::ReductionChain(source_id));

                // Send a reduction job so that if the user opened this panel
                // after already having typed in the input, it populates
                // automatically.
                self.state
                    .push_msg(AppMessage::SendSkiReductionJob(source_id));
            }
            AppMessage::RequestGraphOutput(source_id) => {
                self.spawn_tab(AppTabs::ReductionGraph(source_id));

                // Send a reduction job so that if the user opened this panel
                // after already having typed in the input, it populates
                // automatically.
                self.state
                    .push_msg(AppMessage::SendSkiReductionJob(source_id));
            }
            AppMessage::SetGraphStep(source_id, step_idx) => {
                let Some(source) = self.state.sources.get_mut(&source_id) else {
                    return;
                };

                let is_active = source.is_at_step(step_idx);
                let is_compiled = source.is_cached_for(step_idx);

                if is_active && is_compiled {
                    return;
                }

                source.set_graph_step(step_idx);

                if let Some(graph_history) = &source.reduction_graph
                    && let Some(step_data) = graph_history.get(step_idx)
                {
                    source
                        .compiled_graphs
                        .entry(step_idx)
                        .or_insert_with(|| crate::graph::build_egui_graph(step_data));
                }
            }
            AppMessage::SendSkiReductionJob(source_id) => {
                let wants_steps = self.is_steps_open_for(source_id);
                let wants_graph = self.is_graph_open_for(source_id);

                let Some(source) = self.state.sources.get_mut(&source_id) else {
                    return;
                };

                source.reduction_chain = None;
                source.reduction_graph = None;

                let request = WorkerRequest {
                    source_id,
                    strategy: ApiStrategy::Ski(()),
                    state: WorkerRequestState::Ski {
                        wants_steps,
                        wants_graph,
                    },
                    input: source.ski_input.clone(),
                };

                if let Some(bridge) = &self.worker {
                    bridge.send_job(&request);
                }
            }
            AppMessage::SendLambdaReductionJob(source_id) => {
                let Some(source) = self.state.sources.get_mut(&source_id) else {
                    return;
                };

                source.lambda_output = None;

                let request = WorkerRequest {
                    source_id,
                    strategy: ApiStrategy::Lambda(source.lambda_strategy),
                    state: WorkerRequestState::Lambda { placeholder: false },
                    input: source.lambda_input.clone(),
                };

                if let Some(bridge) = &self.worker {
                    bridge.send_job(&request);
                }
            }
            AppMessage::CloseSourceTab(source_id) => {
                self.tree.retain_tabs(|tab| match tab {
                    AppTabs::ReductionChain(out_id) | AppTabs::ReductionGraph(out_id) => {
                        *out_id != source_id
                    }
                    _ => true,
                });
                self.state.sources.remove(&source_id);
            }
            AppMessage::WorkerJobCompleted(worker_response) => {
                let Some(source) = self.state.sources.get_mut(&worker_response.source_id) else {
                    return;
                };

                if let Some(error) = worker_response.error {
                    source.set_error(error);
                    return;
                }

                source.remove_error();

                if let WorkerResponseState::Ski { steps, graph_nodes } = worker_response.state {
                    source.reduction_chain = steps;
                    source.reduction_graph = graph_nodes;
                    source.active_graph_step = 0;
                    source.compiled_graphs = HashMap::new();

                    if let Some(graphs) = &source.reduction_graph
                        && let Some(first_step) = graphs.first()
                    {
                        let egui_graph = build_egui_graph(first_step);
                        source.compiled_graphs.insert(0, egui_graph);
                    }
                } else if let WorkerResponseState::Lambda { output } = worker_response.state {
                    source.lambda_output = output;
                }
            }
            AppMessage::AddLambdaInput(source_id, visual_comb, time) => {
                let Some(source) = self.state.sources.get_mut(&source_id) else {
                    return;
                };

                let conversion = match visual_comb {
                    VisualComb::S => "(\\x.\\y.\\z.x z(y z))",
                    VisualComb::K => "(\\x.\\y.x)",
                    VisualComb::I => "(\\x.x)",
                    VisualComb::B => "(\\x.\\y.\\z.x(y z))",
                    VisualComb::C => "(\\x.\\y.\\z.x z y)",
                };

                source.lambda_input.push_str(conversion);
                source.last_edited_time = time;
            }
            AppMessage::ConvertSkiToLambda(source_id) => {
                let Some(source) = self.state.sources.get_mut(&source_id) else {
                    return;
                };

                source.converted_lambda_output = Some(String::from("(\\x.x)"));
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

        for (&id, source) in &mut self.state.sources {
            if current_time - source.last_edited_time > debounce_delay {
                to_recompile.push((id, source.kind));
                source.last_edited_time = f64::INFINITY;
            }
        }

        for (id, kind) in to_recompile {
            self.state.push_msg(match kind {
                SourceKind::Ski => AppMessage::SendSkiReductionJob(id),
                SourceKind::Lambda => AppMessage::SendLambdaReductionJob(id),
            });
        }
    }

    #[expect(unused)]
    fn set_theme(&self, _ctx: &egui::Context) {
        match self.state.style {
            // Catpuccin, Gruvbox, Compiler Explorer
            Some(_) => todo!(),
            None => todo!(),
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
