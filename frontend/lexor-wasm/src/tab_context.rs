use egui::{Ui, WidgetText};
use egui_dock::{Style, TabViewer, tab_viewer::OnCloseResponse};
use lexor_reducer::ReductionStrat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppTab {
    Welcome,
    SkiSource { id: usize },
    ReductionOutput { source_id: usize },
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TabContext {
    #[serde(skip)]
    pub style: Option<Style>,

    inputs: HashMap<usize, String>,
    reduction_steps: HashMap<usize, Vec<String>>,
    reduction_results: HashMap<usize, String>,
    last_assigned_key: usize,

    // A queue to remember which parent tabs were just closed
    #[serde(skip)]
    pub closed_source_ids: Vec<usize>,

    // A queue to signal to the UI which tabs need to be opened
    #[serde(skip)]
    pub pending_tabs: Vec<AppTab>,
}

impl TabViewer for TabContext {
    type Tab = AppTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            AppTab::Welcome => "Welcome!".into(),
            AppTab::SkiSource { id } => format!("SKI Source #{id}").into(),
            AppTab::ReductionOutput { source_id } => {
                format!("Reduction Output #{source_id}").into()
            }
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            AppTab::Welcome => self.welcome(ui),
            AppTab::SkiSource { id } => self.ski_source_input(ui, id),
            AppTab::ReductionOutput { source_id } => self.reduction_output(ui, source_id),
        }
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> OnCloseResponse {
        if let AppTab::SkiSource { id } = tab {
            self.closed_source_ids.push(*id);
        }
        OnCloseResponse::Close
    }
}

impl TabContext {
    fn welcome(&self, ui: &mut Ui) {
        ui.heading("Welcome to Lexor!");
    }

    fn ski_source_input(&mut self, ui: &mut Ui, id: usize) {
        ui.vertical(|ui| {
            // Add new bar
            if ui.button("Reduction output").clicked() {
                let reduction_output = self.new_reduction_output(id);
                self.pending_tabs.push(reduction_output);
            }

            // Input field
            let input = self.inputs.entry(id).or_default();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(input);
                if ui.button("Send").clicked() {
                    let mut steps = vec![];

                    let result = lexor_reducer::NF::reduce_with(input, |view| {
                        steps.push(view.to_string());
                    })
                    .unwrap();

                    self.reduction_results.insert(id, result);
                    self.reduction_steps.insert(id, steps);
                }
            });
        });
    }

    fn reduction_output(&mut self, ui: &mut Ui, source_id: usize) {
        let steps = self.reduction_steps.get(&source_id).unwrap();
        let result = self.reduction_results.get(&source_id).unwrap();

        let mut reduction_chain = String::default();
        for (index, step) in steps.iter().enumerate() {
            reduction_chain.push_str(format!("{index}. {step}\n").as_str());
        }

        ui.vertical(|ui| {
            ui.label(reduction_chain);
            ui.label(result)
        });
    }

    pub(crate) fn new_ski_source(&mut self) -> (usize, AppTab) {
        let id = self
            .last_assigned_key
            .checked_add(1)
            .expect("Ran out of ski source keys");

        self.last_assigned_key = id;

        self.inputs.insert(id, String::default());

        (id, AppTab::SkiSource { id })
    }

    pub(crate) fn new_reduction_output(&mut self, id: usize) -> AppTab {
        self.reduction_results.insert(id, String::default());
        self.reduction_steps.insert(id, vec![]);

        AppTab::ReductionOutput { source_id: id }
    }

    pub fn clean_up_ids(&mut self) {
        for id in &self.closed_source_ids {
            self.inputs.remove(&id);
            self.reduction_results.remove(&id);
            self.reduction_steps.remove(&id);
        }
        self.last_assigned_key = *self.inputs.keys().max().unwrap_or_else(|| &0usize);
    }
}
