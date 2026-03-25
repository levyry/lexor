use egui::{ScrollArea, TopBottomPanel, Ui, WidgetText};
use egui_dock::{TabViewer, tab_viewer::OnCloseResponse};
use lexor_api::SourceID;

use crate::{messages::AppMessage, state::AppState, tabs::AppTabs};

pub struct LexorTabViewer<'a> {
    pub state: &'a mut AppState,
}

impl<'a> TabViewer for LexorTabViewer<'a> {
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
            self.state
                .messages
                .borrow_mut()
                .push(AppMessage::CloseSourceTab(*id));
        }
        OnCloseResponse::Close
    }
}

impl<'a> LexorTabViewer<'a> {
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
                            self.state
                                .messages
                                .borrow_mut()
                                .push(AppMessage::RequestChainOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui.button("Reduction Graph").clicked() {
                            self.state
                                .messages
                                .borrow_mut()
                                .push(AppMessage::RequestGraphOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                });
            });

            let input = self.state.inputs.entry(id).or_default();

            if ui.text_edit_singleline(input).changed() {
                self.state.last_edited_time.insert(id, ui.input(|i| i.time));
            }
        });
    }

    fn reduction_output_view(&self, ui: &mut Ui, source_id: SourceID) {
        let steps = self
            .state
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
}
