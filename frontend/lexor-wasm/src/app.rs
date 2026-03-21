#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::vec;

use crate::{
    messages::{AppMessage, SourceType},
    settings::Settings,
    tab_viewer::AppTabViewer,
    tabs::AppTabs,
};
use egui::{CentralPanel, Frame, TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use lexor_api::SourceID;
use lexor_reducer::ReductionStrat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct MyApp {
    tabs: AppTabViewer,
    settings: Settings,
    tree: DockState<AppTabs>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tabs = AppTabViewer::default();
        let ski_tab = tabs.new_ski_source();
        let AppTabs::SkiSource(id) = ski_tab else {
            unreachable!()
        };
        let reduction_tab = tabs.new_reduction_output(id);

        let mut tree = DockState::new(vec![AppTabs::Welcome]);

        let [_welcome_node, ski_node] =
            tree.main_surface_mut()
                .split_above(NodeIndex::root(), 0.3, vec![ski_tab]);

        let [_, _] = tree
            .main_surface_mut()
            .split_right(ski_node, 0.5, vec![reduction_tab]);

        Self {
            tree,
            tabs,
            settings: Settings::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("egui_dock::MenuBar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| self.add_menubar(ui))
        });

        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                let style = self
                    .tabs
                    .style
                    .get_or_insert_with(|| Style::from_egui(ui.style()))
                    .clone();

                // Display view
                DockArea::new(&mut self.tree)
                    .style(style)
                    .show_inside(ui, &mut self.tabs);

                // Debouncer logic
                let current_time = ctx.input(|i| i.time);
                let debounce_delay = 0.5;

                // Find all tabs that need to be recompiled and push the
                // message to run the reduction on them
                let recompiled_tabs: Vec<SourceID> = self
                    .tabs
                    .last_edited_time
                    .iter()
                    .filter(|&(_id, last_time)| current_time - last_time > debounce_delay)
                    .map(|(&id, _)| id)
                    .collect();

                for id in recompiled_tabs {
                    self.tabs.last_edited_time.remove(&id);
                    if self.tabs.inputs.contains_key(&id) {
                        self.tabs.messages.push(AppMessage::RunReduction(id));
                    }
                }

                // Handle the message queue
                let messages = std::mem::take(&mut self.tabs.messages);

                for msg in messages {
                    match msg {
                        AppMessage::RequestNewSource(SourceType::Ski) => {
                            let tab = self.tabs.new_ski_source();
                            self.spawn_tab(tab);
                        }
                        AppMessage::RequestNewSource(SourceType::Lambda) => {
                            todo!()
                        }
                        AppMessage::RequestChainOutput(source_id) => {
                            let tab = self.tabs.new_reduction_output(source_id);
                            self.spawn_tab(tab);
                        }
                        AppMessage::RunReduction(source_id) => {
                            let input = self
                                .tabs
                                .inputs
                                .get(&source_id)
                                .expect("SourceID not found while trying to run reduction");

                            let mut steps = String::default();

                            if !input.is_empty() {
                                let mut counter: u64 = 0;
                                steps.push_str(format!("{counter}. {input}\n").as_str());
                                lexor_reducer::NF::reduce_with(input, |view| {
                                    counter = counter.saturating_add(1);
                                    steps.push_str(format!("{counter}. {view}\n").as_str());
                                })
                                .expect("Reduction failed");
                            }

                            self.tabs.reduction_steps.insert(source_id, steps);
                        }
                        AppMessage::CloseSourceTab(source_id) => {
                            self.tree.retain_tabs(|tab| match tab {
                                AppTabs::ReductionChain(out_id) => *out_id != source_id,
                                _ => true,
                            });
                            self.tabs.inputs.remove(&source_id);
                            self.tabs.reduction_steps.remove(&source_id);
                            self.tabs.last_assigned_key =
                                *self.tabs.inputs.keys().max().unwrap_or(&0usize);
                        }
                        AppMessage::RequestGraphOutput(_source_id) => todo!(),
                    }
                }
            });
    }
}

impl MyApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage
            && let Some(state) = eframe::get_value(storage, eframe::APP_KEY)
        {
            return state;
        }

        Self::default()
    }

    fn add_menubar(&mut self, ui: &mut Ui) {
        ui.menu_button("Add", |ui| {
            if ui.button("SKI source").clicked() {
                self.tabs
                    .messages
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
}
