use serde::{Deserialize, Serialize};

// TODO: Refactor later so its not a field of bools
#[derive(Serialize, Deserialize)]
pub struct Settings {
    // TODO: Look deeper into settings
    show_close_buttons: bool,
    show_add_buttons: bool,
    draggable_tabs: bool,
    show_tab_name_on_hover: bool,
    show_leaf_close_all: bool,
    show_leaf_collapse: bool,
    show_secondary_button_hint: bool,
    secondary_button_on_modifier: bool,
    secondary_button_context_menu: bool,
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
        }
    }
}
