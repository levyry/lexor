use egui::{TopBottomPanel, Ui, WidgetText};
use egui_dock::{TabViewer, tab_viewer::OnCloseResponse};
use egui_graphs::SettingsNavigation;
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
            AppTabs::Welcome => welcome_view(ui),
            AppTabs::SkiSource(id) => self.ski_source_view(ui, id),
            AppTabs::ReductionChain(source_id) => self.reduction_chain_view(ui, source_id),
            AppTabs::ReductionGraph(source_id) => self.reduction_graph_view(ui, source_id),
        }
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> OnCloseResponse {
        if let AppTabs::SkiSource(id) = tab {
            self.state.push_msg(AppMessage::CloseSourceTab(*id));
        }
        OnCloseResponse::Close
    }
}

fn welcome_view(ui: &mut Ui) {
    ui.heading("Welcome to Lexor!");
    ui.label("Lexor is a compiler and VM for lambda calculus targeting combinators. It provides intuitive visualizations for reduction rules and abstract syntax trees. For a list of full capabilities, check out the Help page.");
}

impl LexorTabViewer<'_> {
    fn ski_source_view(&mut self, ui: &mut Ui, id: SourceID) {
        ui.vertical(|ui| {
            let panel_id = egui::Id::new("source_top_panel").with(id);

            TopBottomPanel::top(panel_id).show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("Font size", |ui| {
                        let font_size = self.state.settings.source_font_sizes.get_mut(&id).unwrap();
                        ui.add(egui::Slider::new(font_size, 8.0..=30.0).integer());
                    });
                    ui.menu_button("Add new...", |ui| {
                        if ui.button("Reduction Chain").clicked() {
                            self.state.push_msg(AppMessage::RequestChainOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui.button("Reduction Graph").clicked() {
                            self.state.push_msg(AppMessage::RequestGraphOutput(id));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                });
            });

            // ui.label("test");

            TopBottomPanel::bottom(egui::Id::new("source_bottom_panel").with(id)).show_inside(
                ui,
                |ui| {
                    let input = self.state.inputs.entry(id).or_default();

                    let input_response = egui::TextEdit::singleline(input)
                        .font(egui::FontId::proportional(
                            *self.state.settings.source_font_sizes.get(&id).unwrap(),
                        ))
                        .desired_width(f32::INFINITY)
                        .show(ui)
                        .response;

                    if input_response.changed() {
                        self.state.last_edited_time.insert(id, ui.input(|i| i.time));
                    }
                },
            );
        });
    }

    fn reduction_chain_view(&self, ui: &mut Ui, source_id: SourceID) {
        if let Some(response) = self.state.reduction_steps.get(&source_id)
            && let Some(steps) = response
        {
            if steps.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("Input field has no content.");
                });
                return;
            }

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
                            let is_active =
                                self.state.active_graph_step.get(&source_id) == Some(&index);

                            // TODO: Only make the label clickable if a graph
                            // view is currently open.
                            let line_label = format!("{next_index:3}.");
                            if ui
                                .selectable_label(
                                    is_active,
                                    egui::RichText::new(line_label).monospace(),
                                )
                                .clicked()
                            {
                                self.state
                                    .push_msg(AppMessage::SetGraphStep(source_id, index));
                            }

                            LexorTabViewer::render_steps(ui, step);
                        });
                    }
                });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Compiling...");
            });
        }
    }

    pub fn reduction_graph_view(&mut self, ui: &mut egui::Ui, source_id: SourceID) {
        let current_step = *self.state.active_graph_step.get(&source_id).unwrap_or(&0);
        self.graph_controls_view(ui, source_id, current_step);
        self.graph_view(ui, source_id, current_step);
    }

    fn graph_view(&mut self, ui: &mut Ui, source_id: SourceID, current_step: usize) {
        if let Some(graph_cache) = self.state.compiled_graphs.get_mut(&source_id)
            && let Some(graph) = graph_cache.get_mut(&current_step)
        {
            ui.set_min_size(egui::vec2(200.0, 200.0));
            let available = ui.available_size();

            // TODO: Refactor this in the future
            if available.x > 100.0 && available.y > 100.0 {
                let id = Some(format!("ast_view_{source_id}"));

                let tracker_id = egui::Id::new("step_tracker").with(source_id);
                let previous_step =
                    ui.data(|d| d.get_temp::<usize>(tracker_id).unwrap_or(usize::MAX));

                if previous_step != current_step {
                    egui_graphs::reset_layout::<egui_graphs::LayoutStateHierarchical>(
                        ui,
                        id.clone(),
                    );
                    ui.data_mut(|d| d.insert_temp(tracker_id, current_step));
                }

                // Settings navigation
                let mut nav_settings = SettingsNavigation::default();
                let node_count = graph.node_count();
                let padding = if node_count <= 1 { 4.0 } else { 0.1 };
                nav_settings = nav_settings.with_fit_to_screen_padding(padding);

                let mut graph_view = egui_graphs::GraphView::<
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    egui_graphs::LayoutStateHierarchical,
                    egui_graphs::LayoutHierarchical,
                >::new(graph)
                .with_navigations(&nav_settings)
                .with_id(id);

                ui.add(&mut graph_view);
            } else {
                ui.allocate_space(available);
            }

            ui.ctx().request_repaint();
        } else {
            ui.centered_and_justified(|ui| {
                if let Some(response) = self.state.reduction_graph.get(&source_id)
                    && let Some(graph_history) = response
                    && graph_history.get(current_step).is_some()
                {
                    self.state
                        .push_msg(AppMessage::SetGraphStep(source_id, current_step));
                    ui.label("Compiling graph...");
                } else {
                    ui.label("Input field has no content.");
                }
            });
        }
    }

    fn graph_controls_view(&self, ui: &mut Ui, source_id: SourceID, mut current_step: usize) {
        if let Some(response) = self.state.reduction_graph.get(&source_id)
            && let Some(graph_history) = response
            && !graph_history.is_empty()
        {
            let max_step = graph_history.len().saturating_sub(1);

            ui.horizontal(|ui| {
                if ui.button("⬅").clicked() && current_step > 0 {
                    self.state.push_msg(AppMessage::SetGraphStep(
                        source_id,
                        current_step.saturating_sub(1),
                    ));
                }
                if ui.button("➡").clicked() && current_step < max_step {
                    self.state.push_msg(AppMessage::SetGraphStep(
                        source_id,
                        current_step.saturating_add(1),
                    ));
                }

                let slider = egui::Slider::new(&mut current_step, 0..=max_step).text("Step");
                if ui.add(slider).changed() {
                    self.state
                        .push_msg(AppMessage::SetGraphStep(source_id, current_step));
                }
            });

            ui.separator();

            if ui.ui_contains_pointer() {
                if ui.input(|i| {
                    i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::ArrowUp)
                }) && current_step > 0
                {
                    self.state.push_msg(AppMessage::SetGraphStep(
                        source_id,
                        current_step.saturating_sub(1),
                    ));
                }
                if ui.input(|i| {
                    i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::ArrowDown)
                }) && current_step < max_step
                {
                    self.state.push_msg(AppMessage::SetGraphStep(
                        source_id,
                        current_step.saturating_add(1),
                    ));
                }
            }
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
                        let (head, _bg, outline) = Self::get_redex_colors(*comb);
                        egui::Frame::new()
                            .fill(head)
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
                        let (_head, bg, outline) = Self::get_redex_colors(*comb);
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

    #[must_use]
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
}
