use std::{cell::RefCell, collections::HashMap, rc::Rc};

use egui_dock::Style;
use lexor_api::{SourceID, source_id::SourceKind};
use serde::{Deserialize, Serialize};

use crate::{messages::AppMessage, settings::Settings, source::Source, tab_viewer::AppTabs};

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    pub sources: HashMap<SourceID, Source>,
    pub last_assigned_id_inner: usize,
    pub style: Option<Style>,
    pub settings: Settings,

    #[serde(skip)]
    pub messages: Rc<RefCell<Vec<AppMessage>>>,
}

impl AppState {
    pub fn new_source(&mut self, kind: SourceKind) -> AppTabs {
        let id = self
            .last_assigned_id_inner
            .checked_add(1)
            .expect("Ran out of ski source keys");

        self.last_assigned_id_inner = id;

        let id = SourceID(id);

        self.sources.insert(id, Source::new(kind));

        self.settings.source_font_sizes.insert(id, 12.0);

        AppTabs::Source(id)
    }

    pub fn push_msg(&self, msg: AppMessage) {
        self.messages.borrow_mut().push(msg);
    }
}
