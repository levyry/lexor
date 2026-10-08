use std::collections::{HashMap, VecDeque};

use crate::engineview::EngineView;
use crate::tokenize::EngineToken;
use crate::{
    core::{
        arena::Arena,
        engine::Engine,
        node::{Node, NodeComb, NodeKey},
        visitor::Visitor,
    },
    graphred::ReductionError,
};
use lexor_core::literal::Literal;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateNode {
    pub id: usize,
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateEdge {
    pub from: usize,
    pub to: usize,
    pub is_normal_order: bool,
    pub transition_tokens: Vec<EngineToken>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateSpace {
    pub nodes: Vec<StateNode>,
    pub edges: Vec<StateEdge>,
}

pub struct RedexVisitor<'a> {
    arena: &'a Arena,
    current_path: Vec<NodeKey>,
    redexes: Vec<Vec<NodeKey>>,
}

impl<'a> Visitor<'a> for RedexVisitor<'a> {
    fn arena(&self) -> &'a Arena {
        self.arena
    }

    fn visit_app(&mut self, key: NodeKey, lhs: NodeKey, rhs: NodeKey) {
        // Continue down the spine
        self.current_path.push(key);
        self.visit_node(lhs);
        self.current_path.pop();

        // Arguments has its own path
        let old_path = std::mem::take(&mut self.current_path);
        self.visit_node(rhs);
        self.current_path = old_path;
    }

    fn visit_comb(&mut self, key: NodeKey, comb: NodeComb) {
        if self.current_path.len() >= comb.arity() {
            let mut full_spine = self.current_path.clone();
            full_spine.push(key);
            self.redexes.push(full_spine);
        }
    }

    fn visit_prim(&mut self, key: NodeKey, prim: &'a Literal) {
        if let Literal::Op(op) = prim
            && let arg_count = op.arity()
            && self.current_path.len() >= arg_count
        {
            let is_valid = self
                .current_path
                .iter()
                .rev()
                .take(arg_count)
                .all(|&app_key| {
                    self.arena.get_arg(app_key).is_ok_and(|arg_key| {
                        matches!(self.arena().get(arg_key), Ok(Node::Prim(Literal::Num(_))))
                    })
                });

            if is_valid {
                let mut full_spine = self.current_path.clone();
                full_spine.push(key);
                self.redexes.push(full_spine);
            }
        }
    }
}

pub fn generate_state_space(input: &str) -> Result<StateSpace, ReductionError> {
    let root = lexor_parser::parse_ski(input)?;
    let initial_engine = Engine::from_tree(root);

    let mut states = Vec::new();
    let mut edges = Vec::new();
    let mut state_map: HashMap<String, usize> = HashMap::new();
    let mut queue = VecDeque::new();

    let initial_expr = format!("{}", EngineView::from_engine(&initial_engine));

    states.push(StateNode {
        id: 0,
        expression: initial_expr.clone(),
    });

    state_map.insert(initial_expr, 0);
    queue.push_back((0, initial_engine, true));

    while let Some((state_id, engine, is_on_normal_path)) = queue.pop_front() {
        let mut visitor = RedexVisitor {
            arena: &engine.arena,
            current_path: Vec::new(),
            redexes: Vec::new(),
        };

        visitor.visit_node(engine.root);

        for (idx, spine) in visitor.redexes.into_iter().enumerate() {
            let mut next_engine = engine.clone();

            next_engine.spine = spine.clone();
            let _ = next_engine.reduce()?;
            next_engine.spine.clear();

            let next_expr = format!("{}", EngineView::from_engine(&next_engine));

            // Avoid duplicate nodes
            let next_id = if let Some(&id) = state_map.get(&next_expr) {
                id
            } else {
                let id = states.len();

                states.push(StateNode {
                    id,
                    expression: next_expr.clone(),
                });

                let next_is_on_normal_path = is_on_normal_path && idx == 0;

                state_map.insert(next_expr.clone(), id);
                queue.push_back((id, next_engine, next_is_on_normal_path));

                id
            };

            // Avoid duplicate edges
            if !edges
                .iter()
                .any(|e: &StateEdge| e.from == state_id && e.to == next_id)
            {
                // We recreate the exact spine for the transition to tokenize it
                let mut transition_engine = engine.clone();
                transition_engine.spine = spine;
                let transition_tokens = EngineView::from_engine(&transition_engine).tokenize();

                edges.push(StateEdge {
                    from: state_id,
                    to: next_id,
                    is_normal_order: is_on_normal_path && idx == 0,
                    transition_tokens,
                });
            }
        }
    }

    Ok(StateSpace {
        nodes: states,
        edges,
    })
}

#[cfg(test)]
mod tests {
    #![expect(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_generate_state_space_arithmetic() {
        let space = generate_state_space("+ (+ 1 2) (+ 3 4)").unwrap();
        // 1. (+ (+ 1 2) (+ 3 4))
        // 2. (+ 3 (+ 3 4))
        // 3. (+ (+ 1 2) 7)
        // 4. (+ 3 7)
        // 5. 10
        assert_eq!(space.nodes.len(), 5);
        assert_eq!(space.edges.len(), 5);
    }

    #[test]
    fn test_generate_state_space_ski() {
        // S K I 1 -> K 1 (I 1) -> 1
        // K 1 (I 1) -> K 1 1 -> 1
        // Expected nodes:
        // S K I 1
        // K 1 (I 1)
        // K 1 1
        // 1
        let space = generate_state_space("S K I 1").unwrap();
        assert_eq!(space.nodes.len(), 4);
        assert_eq!(space.edges.len(), 4);
    }

    #[test]
    fn test_generate_state_space_multiple_redexes() {
        // I I I
        // (I I) I -> I I -> I
        // Expected nodes:
        // I I I
        // I I
        // I
        let space = generate_state_space("I I I").unwrap();
        assert_eq!(space.nodes.len(), 3);
        assert_eq!(space.edges.len(), 2);
    }
}
