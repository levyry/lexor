use egui_graphs::Graph;
use lexor_api::graph::{ApiGraphNodeKind, NodeData};
use petgraph::stable_graph::StableGraph;
use std::collections::HashMap;

use crate::tab_viewer::LexorTabViewer;

pub fn build_egui_graph(nodes: &[NodeData]) -> Graph<(), ()> {
    let mut pg = StableGraph::new();
    let mut indices = HashMap::new();

    for node in nodes {
        let idx = pg.add_node(());
        indices.insert(node.id, idx);
    }

    // 2. Add Edges
    for node in nodes {
        if let Some(&start_idx) = indices.get(&node.id) {
            for child_id in &node.children {
                if let Some(&end_idx) = indices.get(child_id) {
                    // Graph Reduction DAG magic happens here: multiple edges
                    // can naturally point to the same 'end_idx'!
                    pg.add_edge(start_idx, end_idx, ());
                }
            }
        }
    }

    // 3. Convert to the egui_graphs wrapper
    let mut egui_graph = Graph::from(&pg);

    // 4. Style the nodes
    for node in nodes {
        if let Some(&idx) = indices.get(&node.id) {
            if let Some(n) = egui_graph.node_mut(idx) {
                // Update the visual properties.
                // Note: The exact struct fields might vary slightly depending on your
                // egui_graphs version, but it generally looks like this:
                let label = match &node.kind {
                    ApiGraphNodeKind::App => "@".to_string(),
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
    }

    egui_graph
}
