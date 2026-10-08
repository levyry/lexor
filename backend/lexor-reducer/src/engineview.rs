use lexor_core::literal::Literal;
use slotmap::Key;

use crate::core::{
    engine::Engine,
    node::{Node, NodeComb},
    visitor::Visitor,
};

use core::fmt::{self, Display};
use std::collections::HashSet;

use crate::tokenize::{EngineToken, NodeRole, TokenizeVisitor, TraverseState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EngineGraphNodeKind {
    App,
    Comb(NodeComb),
    Prim(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EngineGraphNode {
    pub id: u64,
    pub kind: EngineGraphNodeKind,
    pub children: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct EngineView<'a>(&'a Engine);

impl<'a> EngineView<'a> {
    #[must_use]
    pub(crate) const fn from_engine(engine: &'a Engine) -> Self {
        Self(engine)
    }

    #[must_use]
    pub const fn stack_depth(self) -> usize {
        self.0.spine.len()
    }

    #[must_use]
    pub fn extract_graph(&self) -> Vec<EngineGraphNode> {
        let mut graph = Vec::new();
        let mut visited = HashSet::new();
        let mut work_stack = vec![self.0.root];

        while let Some(current) = work_stack.pop() {
            let resolved_key = current;

            if !visited.insert(resolved_key) {
                continue;
            }

            if let Ok(node) = self.0.arena.get(resolved_key) {
                let id = resolved_key.data().as_ffi();

                match node {
                    Node::Comb(comb) => {
                        graph.push(EngineGraphNode {
                            id,
                            kind: EngineGraphNodeKind::Comb(*comb),
                            children: vec![],
                        });
                    }
                    Node::Prim(prim) => {
                        graph.push(EngineGraphNode {
                            id,
                            kind: EngineGraphNodeKind::Prim(*prim),
                            children: vec![],
                        });
                    }
                    Node::App(lhs, rhs) => {
                        let id_lhs = lhs.data().as_ffi();
                        let id_rhs = rhs.data().as_ffi();

                        graph.push(EngineGraphNode {
                            id,
                            kind: EngineGraphNodeKind::App,
                            children: vec![id_lhs, id_rhs],
                        });

                        work_stack.push(*rhs);
                        work_stack.push(*lhs);
                    }
                    Node::Indirection(_) => unreachable!("arena.get() hides indirections"),
                }
            }
        }

        graph
    }

    #[must_use]
    pub fn tokenize(&self) -> Vec<EngineToken> {
        let spine = &self.0.spine;
        let arena = &self.0.arena;

        let mut redex_parent = None;
        let mut redex_args = Vec::with_capacity(3);
        let mut active_comb = None;

        if let Some(&head_key) = spine.last()
            && let Ok(Node::Comb(comb)) = arena.get(head_key)
            && spine.len() > comb.arity()
        {
            redex_parent = spine.get(spine.len().saturating_sub(2)).copied();
            active_comb = Some(*comb);

            let arity = comb.arity();

            let index_of_last = spine.len().saturating_sub(1);
            let arguments_range = index_of_last.saturating_sub(arity)..index_of_last;

            spine
                .get(arguments_range)
                .expect("we used arity to stay in bounds")
                .iter()
                .rev()
                .for_each(|&app_key| {
                    if let Ok(Node::App(_, rhs)) = arena.get(app_key) {
                        redex_args.push(*rhs);
                    }
                });
        }

        let mut tokens = Vec::new();
        let mut tokenizer = TokenizeVisitor {
            arena,
            state: TraverseState {
                current_role: NodeRole::Normal,
                is_rhs_of_app: false,
                is_redex_head: false,
            },
            redex_parent,
            redex_args: &redex_args,
            active_comb,
            tokens: &mut tokens,
            root: self.0.root,
        };

        tokenizer.visit_node(self.0.root);
        tokens
    }
}

impl Display for EngineView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in self.tokenize() {
            write!(f, "{}", token.text)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::unwrap_used)]
    use super::*;
    use lexor_parser::parse_ski;

    #[test]
    fn test_engine_view_roles() {
        let root = parse_ski("S K I 1 2").unwrap();
        let mut engine = Engine::from_tree(root);
        engine.unwind(engine.root).unwrap();

        let view = EngineView::from_engine(&engine);
        let tokens = view.tokenize();

        let get_role =
            |target: &str| -> NodeRole { tokens.iter().find(|t| t.text == target).unwrap().role };

        assert_eq!(get_role("S"), NodeRole::RedexHead(NodeComb::S));
        assert_eq!(get_role("K"), NodeRole::RedexArg(NodeComb::S, 1));
        assert_eq!(get_role("I"), NodeRole::RedexArg(NodeComb::S, 2));
        assert_eq!(get_role("1"), NodeRole::RedexArg(NodeComb::S, 3));
        assert_eq!(get_role("2"), NodeRole::Normal);
    }

    #[test]
    fn test_engine_view_infix_formatting() {
        let root1 = parse_ski("+ 1 (* 2 3)").unwrap();
        let engine1 = Engine::from_tree(root1);
        let view1 = EngineView::from_engine(&engine1);
        assert_eq!(format!("{view1}"), "1 + (2 * 3)");

        let root2 = parse_ski("+ (+ 1 2) 3").unwrap();
        let engine2 = Engine::from_tree(root2);
        let view2 = EngineView::from_engine(&engine2);
        assert_eq!(format!("{view2}"), "(1 + 2) + 3");

        let root3 = parse_ski("* (+ 1 2) (- 3 4)").unwrap();
        let engine3 = Engine::from_tree(root3);
        let view3 = EngineView::from_engine(&engine3);
        assert_eq!(format!("{view3}"), "(1 + 2) * (3 - 4)");
    }

    #[test]
    fn test_engine_view_parens_standard() {
        let root = parse_ski("S (K I 1)").unwrap();
        let engine = Engine::from_tree(root);
        let view = EngineView::from_engine(&engine);
        assert_eq!(format!("{view}"), "S(KI1)");
    }

    #[test]
    fn test_engine_view_parens_infix() {
        let root = parse_ski("S (K I + 2)").unwrap();
        let engine = Engine::from_tree(root);
        let view = EngineView::from_engine(&engine);
        assert_eq!(format!("{view}"), "S(KI + 2)");
    }
}
