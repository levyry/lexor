use std::collections::HashMap;

use lexor_api::SourceID;
use serde::{Deserialize, Serialize};

// TODO: Refactor later so its not a field of bools
#[derive(Serialize, Deserialize)]
pub struct Settings {
    // TODO: Look deeper into settings
    pub show_close_buttons: bool,
    pub show_add_buttons: bool,
    pub draggable_tabs: bool,
    pub show_tab_name_on_hover: bool,
    pub show_leaf_close_all: bool,
    pub show_leaf_collapse: bool,
    pub show_secondary_button_hint: bool,
    pub secondary_button_on_modifier: bool,
    pub secondary_button_context_menu: bool,
    pub source_font_sizes: HashMap<SourceID, f32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_leaf_close_all: true,
            show_leaf_collapse: true,
            show_secondary_button_hint: true,
            secondary_button_on_modifier: true,
            secondary_button_context_menu: true,
            show_close_buttons: true,
            show_add_buttons: false,
            draggable_tabs: true,
            show_tab_name_on_hover: false,
            source_font_sizes: HashMap::new(),
        }
    }
}
