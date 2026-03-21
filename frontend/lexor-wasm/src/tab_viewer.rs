use egui::{TopBottomPanel, Ui, WidgetText};
use egui_dock::{Style, TabViewer, tab_viewer::OnCloseResponse};
use lexor_api::SourceID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{messages::AppMessage, tabs::AppTabs};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppTabViewer {
    #[serde(skip)]
    pub style: Option<Style>,

    pub inputs: HashMap<SourceID, String>,
    pub reduction_steps: HashMap<SourceID, String>,
    pub last_assigned_key: SourceID,

    #[serde(skip)]
    pub last_edited_time: HashMap<SourceID, f64>,

    #[serde(skip)]
    pub messages: Vec<AppMessage>,
}

impl TabViewer for AppTabViewer {
    type Tab = AppTabs;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            AppTabs::Welcome => "Welcome!".to_owned(),
            AppTabs::SkiSource(id) => format!("SKI Source #{id}"),
            AppTabs::ReductionChain(source_id) => format!("Reduction Chain #{source_id}"),
            AppTabs::ReductionGraph(source_id) => format!("Reduction Graph #{source_id}"),
        }
        .into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            AppTabs::Welcome => self.welcome_view(ui),
            AppTabs::SkiSource(id) => self.ski_source_view(ui, id),
            AppTabs::ReductionChain(source_id) => self.reduction_output_view(ui, source_id),
            AppTabs::ReductionGraph(source_id) => self.reduction_graph_view(ui, source_id),
        }
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> OnCloseResponse {
        if let AppTabs::SkiSource(id) = tab {
            self.messages.push(AppMessage::CloseSourceTab(*id));
        }
        OnCloseResponse::Close
    }
}

impl AppTabViewer {
    fn welcome_view(&self, ui: &mut Ui) {
        ui.heading("Welcome to Lexor!");
    }

    fn ski_source_view(&mut self, ui: &mut Ui, id: SourceID) {
        ui.vertical(|ui| {
            let panel_id = egui::Id::new("source_top_panel").with(id);

            TopBottomPanel::top(panel_id).show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("Add new...", |ui| {
                        if ui.button("Reduction Chain").clicked() {
                            self.messages.push(AppMessage::RequestChainOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui.button("Reduction Graph").clicked() {
                            self.messages.push(AppMessage::RequestGraphOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                });
            });

            let input = self.inputs.entry(id).or_default();

            if ui.text_edit_singleline(input).changed() {
                self.last_edited_time.insert(id, ui.input(|i| i.time));
            }
        });
    }

    fn reduction_output_view(&self, ui: &mut Ui, source_id: SourceID) {
        let steps = self
            .reduction_steps
            .get(&source_id)
            .expect("Reduction steps not found");

        ui.vertical(|ui| {
            ui.label(steps);
        });
    }

    #[expect(unused)]
    pub fn reduction_graph_view(&self, ui: &mut Ui, source_id: SourceID) {
        todo!()
    }

    pub fn new_ski_source(&mut self) -> AppTabs {
        let id = self
            .last_assigned_key
            .checked_add(1)
            .expect("Ran out of ski source keys");

        self.last_assigned_key = id;

        self.inputs.insert(id, String::default());

        AppTabs::SkiSource(id)
    }

    pub fn new_reduction_output(&mut self, id: usize) -> AppTabs {
        self.reduction_steps.insert(id, String::default());
        AppTabs::ReductionChain(id)
    }
}
