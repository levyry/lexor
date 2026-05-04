use std::{cell::RefCell, collections::HashMap, rc::Rc};

use egui_dock::Style;
use lexor_api::{GraphStep, ReductionStep, SourceID};
use serde::{Deserialize, Serialize};

use crate::{graph::LexorGraph, messages::AppMessage, settings::Settings, tab_viewer::AppTabs};

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    pub inputs: HashMap<SourceID, String>,
    pub reduction_steps: HashMap<SourceID, Option<Vec<ReductionStep>>>,
    pub reduction_graph: HashMap<SourceID, Option<Vec<GraphStep>>>,
    pub last_edited_time: HashMap<SourceID, f64>,
    pub active_graph_step: HashMap<SourceID, usize>,

    pub last_assigned_id_inner: usize,
    pub style: Option<Style>,
    pub settings: Settings,

    #[serde(skip)]
    pub messages: Rc<RefCell<Vec<AppMessage>>>,

    #[serde(skip)]
    pub compiled_graphs: HashMap<SourceID, HashMap<usize, LexorGraph>>,
}

impl AppState {
    pub fn new_ski_source(&mut self) -> AppTabs {
        let id = self
            .last_assigned_id_inner
            .checked_add(1)
            .expect("Ran out of ski source keys");

        self.last_assigned_id_inner = id;

        let id = SourceID(id);

        self.inputs.insert(id, String::new());
        self.settings.source_font_sizes.insert(id, 12.0);

        AppTabs::SkiSource(id)
    }

    pub fn new_reduction_output(&mut self, id: SourceID) -> AppTabs {
        self.reduction_steps.insert(id, None);
        AppTabs::ReductionChain(id)
    }

    pub fn new_graph_output(&mut self, id: SourceID) -> AppTabs {
        self.reduction_graph.insert(id, None);
        AppTabs::ReductionGraph(id)
    }

    pub fn push_msg(&self, msg: AppMessage) {
        self.messages.borrow_mut().push(msg);
    }
}
