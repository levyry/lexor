use egui_graphs::Graph;
use lexor_api::graph::{ApiGraphNodeKind, NodeData};
use petgraph::stable_graph::StableGraph;
use std::collections::HashMap;

use crate::{node_style::CustomNodeShape, tab_viewer::LexorTabViewer};

pub type LexorGraph = Graph<(), (), petgraph::Directed, u32, CustomNodeShape>;

// TODO: refactor
#[must_use]
pub fn build_egui_graph(nodes: &[NodeData]) -> LexorGraph {
    let mut pg = StableGraph::new();
    let mut node_indices = HashMap::new();
    let mut edge_indices = vec![];

    for node in nodes {
        node_indices.insert(node.id, pg.add_node(()));
    }

    for node in nodes {
        if let Some(&start_idx) = node_indices.get(&node.id) {
            for child_id in &node.children {
                if let Some(&end_idx) = node_indices.get(child_id) {
                    edge_indices.push(pg.add_edge(start_idx, end_idx, ()));
                }
            }
        }
    }

    let mut egui_graph = Graph::from(&pg);

    for node in nodes {
        if let Some(&idx) = node_indices.get(&node.id)
            && let Some(n) = egui_graph.node_mut(idx)
        {
            let label = match &node.kind {
                ApiGraphNodeKind::App => "@".to_owned(),
                ApiGraphNodeKind::Comb(c) => format!("{c:?}"),
            };

            let color = match &node.kind {
                ApiGraphNodeKind::App => egui::Color32::from_rgb(60, 60, 60),
                ApiGraphNodeKind::Comb(c) => LexorTabViewer::get_redex_colors(*c).0,
            };

            n.set_label(label);
            n.set_color(color);
        }
    }

    for idx in edge_indices {
        if let Some(e) = egui_graph.edge_mut(idx) {
            e.set_label(String::new());
        }
    }

    egui_graph
}
