use egui::{TopBottomPanel, Ui, WidgetText};
use egui_dock::{TabViewer, tab_viewer::OnCloseResponse};
use lexor_api::{
    SourceID,
    visual::{RenderToken, TokenStyle, VisualComb},
};
use serde::{Deserialize, Serialize};

use crate::{messages::AppMessage, state::AppState};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppTabs {
    Welcome,
    SkiSource(SourceID),
    ReductionChain(SourceID),
    ReductionGraph(SourceID),
}

pub struct LexorTabViewer<'a> {
    pub state: &'a mut AppState,
}

impl TabViewer for LexorTabViewer<'_> {
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

impl LexorTabViewer<'_> {
    fn welcome_view(&self, ui: &mut Ui) {
        ui.heading("Welcome to Lexor!");
        ui.label("Lexor is a compiler and VM for lambda calculus targeting combinators. It provides intuitive visualizations for reduction rules and abstract syntax trees. For a list of full capabilities, check out the Help page.");
    }

    fn ski_source_view(&mut self, ui: &mut Ui, id: SourceID) {
        ui.vertical(|ui| {
            let panel_id = egui::Id::new("source_top_panel").with(id);

            TopBottomPanel::top(panel_id).show_inside(ui, |ui| {
                ui.horizontal_top(|ui| {
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
        let row_height = 20.0;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, steps.len(), |ui, row_range| {
                for index in row_range {
                    let step = steps.get(index).expect(
                        "We gave steps.len() in show_rows, so this should stay in bounds always.",
                    );

                    let next_index = index.saturating_add(1);

                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("{next_index:3}."))
                                .monospace()
                                .color(egui::Color32::DARK_GRAY),
                        ));

                        LexorTabViewer::render_steps(ui, step);
                    });
                }
            });
    }

    pub fn reduction_graph_view(&mut self, ui: &mut egui::Ui, source_id: SourceID) {
        if let Some(graph) = self.state.compiled_graphs.get_mut(&source_id) {
            type L = egui_graphs::LayoutHierarchical;
            type S = egui_graphs::LayoutStateHierarchical;

            // 1. Give this specific graph instance a UNIQUE ID!
            // This stops egui from sharing corrupted layout states across tabs or updates.
            let id = Some(format!("ast_{}_{}", source_id, graph.g().node_count()));

            // 2. Fetch and set the state using the unique ID
            let state = egui_graphs::get_layout_state::<S>(ui, id.clone());
            egui_graphs::set_layout_state::<S>(ui, state, id.clone());

            // 3. Bind the unique ID to the GraphView
            let mut graph_view =
                egui_graphs::GraphView::<_, _, _, _, _, _, S, L>::new(graph).with_id(id);

            ui.add(&mut graph_view);

            // 4. Force egui to render subsequent frames immediately.
            // Frame 1: Widget sizes are 0. Layout squashes.
            // Frame 2: Real sizes are known. Layout expands.
            ui.ctx().request_repaint();
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Waiting for graph data...");
            });
        }
    }

    pub const fn get_redex_colors(
        comb: VisualComb,
    ) -> (egui::Color32, egui::Color32, egui::Color32) {
        match comb {
            VisualComb::S => (
                // Red
                egui::Color32::from_rgb(150, 50, 50),
                egui::Color32::from_rgb(50, 20, 20),
                egui::Color32::from_rgb(255, 100, 100),
            ),
            VisualComb::K => (
                // Blue
                egui::Color32::from_rgb(50, 100, 150),
                egui::Color32::from_rgb(20, 40, 60),
                egui::Color32::from_rgb(100, 150, 255),
            ),
            VisualComb::I => (
                // Green
                egui::Color32::from_rgb(50, 150, 50),
                egui::Color32::from_rgb(20, 60, 20),
                egui::Color32::from_rgb(100, 255, 100),
            ),
            VisualComb::B => (
                // Orange
                egui::Color32::from_rgb(150, 100, 50),
                egui::Color32::from_rgb(60, 40, 20),
                egui::Color32::from_rgb(255, 180, 100),
            ),
            VisualComb::C => (
                // Purple
                egui::Color32::from_rgb(150, 50, 150),
                egui::Color32::from_rgb(60, 20, 60),
                egui::Color32::from_rgb(255, 100, 255),
            ),
        }
    }

    // SAFETY: The if conditions check for tokens.len(), so it should
    //         stay bounded. Same reasoning can show that the arithmetic
    //         expression will never underflow.
    #[expect(clippy::arithmetic_side_effects)]
    #[expect(clippy::indexing_slicing)]
    fn render_steps(ui: &mut egui::Ui, tokens: &[RenderToken]) {
        let font_size = 12.0;
        let max_tokens_to_render = 300;

        let (display_tokens, truncated_count) = if tokens.len() > max_tokens_to_render {
            (
                &tokens[..max_tokens_to_render],
                tokens.len() - max_tokens_to_render,
            )
        } else {
            (tokens, 0)
        };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.spacing_mut().item_spacing.y = 4.0;

            for chunk in display_tokens.chunk_by(|a, b| a.style == b.style) {
                let style = &chunk[0].style;
                let text: String = chunk.iter().map(|t| t.text.as_str()).collect();
                let node_key = chunk[0].node_key;

                let response = match style {
                    TokenStyle::Normal => {
                        egui::Frame::new()
                            .inner_margin(egui::Margin::symmetric(0, 2))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(text)
                                        .color(egui::Color32::GRAY)
                                        .monospace()
                                        .size(font_size),
                                )
                            })
                            .response
                    }
                    TokenStyle::RedexHead(comb) => {
                        // Pass the combinator to get specific colors
                        let (bg, _, outline) = Self::get_redex_colors(*comb);
                        egui::Frame::new()
                            .fill(bg)
                            .stroke(egui::Stroke::new(1.0_f32, outline))
                            .corner_radius(1.0)
                            .inner_margin(egui::Margin::symmetric(4, 2))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(text)
                                        .color(egui::Color32::WHITE)
                                        .monospace()
                                        .size(font_size),
                                )
                            })
                            .response
                    }
                    TokenStyle::RedexBody(comb, _arg_idx) => {
                        // Pass the combinator to get specific colors
                        let (_, bg, outline) = Self::get_redex_colors(*comb);
                        egui::Frame::new()
                            .fill(bg)
                            .stroke(egui::Stroke::new(1.0_f32, outline))
                            .corner_radius(1.0)
                            .inner_margin(egui::Margin::symmetric(4, 2))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(text)
                                        .color(egui::Color32::LIGHT_GRAY)
                                        .monospace()
                                        .size(font_size),
                                )
                            })
                            .response
                    }
                };

                if response.hovered()
                    && let Some(_key) = node_key
                {
                    // Display AST tooltip...
                }
            }

            // If we truncated, add a muted label indicating how much was hidden
            if truncated_count > 0 {
                ui.label(
                    egui::RichText::new(format!("... ({truncated_count} more tokens)"))
                        .color(egui::Color32::DARK_GRAY)
                        .italics()
                        .size(font_size),
                );
            }
        });
    }
}
