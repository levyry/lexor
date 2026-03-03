#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::vec;

use crate::{
    settings::Settings,
    tab_context::{AppTab, TabContext},
};
use egui::{CentralPanel, Frame, TopBottomPanel, Ui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct MyApp {
    tabs: TabContext,
    settings: Settings,
    tree: DockState<AppTab>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tabs = TabContext::default();
        let (id, ski_tab) = tabs.new_ski_source();
        let reduction_tab = tabs.new_reduction_output(id);

        let mut tree = DockState::new(vec![AppTab::Welcome]);

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
                    .get_or_insert(Style::from_egui(ui.style()))
                    .clone();

                // Display main docker area
                DockArea::new(&mut self.tree)
                    .style(style)
                    .show_inside(ui, &mut self.tabs);

                // Display pending tabs
                let pending = std::mem::take(&mut self.tabs.pending_tabs);
                for tab in pending {
                    self.spawn_tab(tab);
                }

                // Process closed tabs
                if !self.tabs.closed_source_ids.is_empty() {
                    self.tree.retain_tabs(|tab| match tab {
                        AppTab::ReductionOutput { source_id } => {
                            !self.tabs.closed_source_ids.contains(source_id)
                        }
                        _ => true,
                    });

                    self.tabs.clean_up_ids();

                    self.tabs.closed_source_ids.clear();
                }
            });
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                return state;
            }
        }

        Self::default()
    }

    fn add_menubar(&mut self, ui: &mut Ui) {
        ui.menu_button("Add", |ui| {
            if ui.button("SKI source").clicked() {
                let (_id, tab) = self.tabs.new_ski_source();

                self.spawn_tab(tab);

                ui.close();
            }
        });
    }

    fn spawn_tab(&mut self, tab: AppTab) {
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
