use egui::{ScrollArea, TopBottomPanel, Ui, WidgetText};
use egui_dock::{Style, TabViewer, tab_viewer::OnCloseResponse};
use lexor_api::{NodeData, SourceID};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{messages::AppMessage, tabs::AppTabs};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LexorTabViewer {
    #[serde(skip)]
    pub style: Option<Style>,

    pub inputs: HashMap<SourceID, String>,
    pub reduction_steps: HashMap<SourceID, Vec<String>>,
    pub reduction_graph: HashMap<SourceID, Vec<NodeData>>,
    pub last_assigned_key: SourceID,

    #[serde(skip)]
    pub last_edited_time: HashMap<SourceID, f64>,

    #[serde(skip)]
    pub messages: Rc<RefCell<Vec<AppMessage>>>,
}

impl TabViewer for LexorTabViewer {
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
            self.messages
                .borrow_mut()
                .push(AppMessage::CloseSourceTab(*id));
        }
        OnCloseResponse::Close
    }
}

impl LexorTabViewer {
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
                            self.messages
                                .borrow_mut()
                                .push(AppMessage::RequestChainOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui.button("Reduction Graph").clicked() {
                            self.messages
                                .borrow_mut()
                                .push(AppMessage::RequestGraphOutput(id));
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
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);

            ScrollArea::vertical().show_rows(ui, row_height * 0.2, steps.len(), |ui, row_range| {
                for row in row_range {
                    let next = row.saturating_add(1);
                    // SAFETY: Since we gave steps.len() to the function
                    // above, we can never index out of bounds here.
                    #[allow(clippy::indexing_slicing)]
                    ui.label(format!("{}. {}", next, steps[row]));
                }
            });
            ui.label("The result"); // TODO: fix
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
        self.reduction_steps.insert(id, vec![String::new()]);
        AppTabs::ReductionChain(id)
    }
}
