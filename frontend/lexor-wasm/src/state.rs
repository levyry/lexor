use std::{cell::RefCell, collections::HashMap, rc::Rc};

use egui_dock::Style;
use lexor_api::{NodeData, ReductionStep, SourceID, visual::RenderToken};
use serde::{Deserialize, Serialize};

use crate::{messages::AppMessage, tabs::AppTabs};

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    pub inputs: HashMap<SourceID, String>,
    pub reduction_steps: HashMap<SourceID, Vec<ReductionStep>>,
    pub reduction_graph: HashMap<SourceID, Vec<NodeData>>,
    pub last_edited_time: HashMap<SourceID, f64>,
    pub last_assigned_key: usize,

    pub style: AppStyle,

    #[serde(skip)]
    pub messages: Rc<RefCell<Vec<AppMessage>>>,
}

impl AppState {
    pub fn new_ski_source(&mut self) -> AppTabs {
        let id = self
            .last_assigned_key
            .checked_add(1)
            .expect("Ran out of ski source keys");

        self.last_assigned_key = id;

        self.inputs.insert(id, String::new());

        AppTabs::SkiSource(id)
    }

    pub fn new_reduction_output(&mut self, id: usize) -> AppTabs {
        self.reduction_steps.insert(
            id,
            vec![vec![RenderToken {
                text: String::new(),
                style: lexor_api::visual::TokenStyle::Normal,
                node_key: None,
            }]],
        );
        AppTabs::ReductionChain(id)
    }
}
