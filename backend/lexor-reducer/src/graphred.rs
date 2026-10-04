use core::{
    fmt::{self, Debug},
    hash::Hash,
};

use lexor_core::literal::Literal;
use lexor_parser::parse_ski;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    core::{
        engine::{Engine, ReductionState},
        node::Node,
    },
    engineview::EngineView,
};

#[derive(Error, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum ReductionError {
    #[error("Arena error: {0}")]
    Arena(#[from] crate::core::arena::ArenaError),
    #[error("Engine error: {0}")]
    Engine(#[from] crate::core::engine::EngineError),
    #[error("Parsing error: {0}")]
    Parsing(#[from] lexor_parser::ParsingError),
}

type Result<T> = core::result::Result<T, ReductionError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ReductionStrategy {
    NormalForm,
    WeakHeadNormalForm,
}

impl ReductionStrategy {
    pub(crate) fn perform(
        self,
        engine: &mut Engine,
        callback: &mut Option<impl FnMut(EngineView)>,
    ) -> Result<()> {
        match self {
            Self::NormalForm => {
                let mut work_stack = Vec::with_capacity(20);
                work_stack.push(engine.subtree_root);

                while let Some(next_root) = work_stack.pop() {
                    engine.subtree_root = next_root;
                    engine.spine.clear();

                    Self::WeakHeadNormalForm.perform(engine, callback)?;

                    let _head = engine.spine.pop();
                    for &app_node in &engine.spine {
                        if let Ok(Node::App(_lhs, rhs)) = engine.arena.get(app_node) {
                            work_stack.push(*rhs);
                        }
                    }
                }
            }
            Self::WeakHeadNormalForm => {
                let mut prev_reduction = ReductionState::Reduced;

                while prev_reduction == ReductionState::Reduced {
                    let current = if let Some(&end) = engine.spine.last() {
                        match engine.arena.get(end)? {
                            Node::App(lhs, _) => *lhs,
                            _ => end,
                        }
                    } else {
                        engine.subtree_root
                    };

                    engine.unwind(current)?;

                    let is_reducible = if let Some(&head_key) = engine.spine.last() {
                        match engine.arena.get(head_key) {
                            Ok(Node::Comb(comb)) => engine.spine.len() > comb.arity(),
                            Ok(Node::Prim(Literal::Op(op)))
                                if engine.spine.len() > op.arity() =>
                            {
                                let sp_len = engine.spine.len();
                                let inner_key = engine.spine.get(sp_len.saturating_sub(2)).copied();
                                let outer_key = engine.spine.get(sp_len.saturating_sub(3)).copied();
                                let is_num = |mut k| {
                                    while let Ok(Node::Indirection(t)) = engine.arena.get(k) {
                                        k = *t;
                                    }
                                    matches!(engine.arena.get(k), Ok(Node::Prim(Literal::Num(_))))
                                };
                                let arg0_ok = inner_key.and_then(|k| engine.arena.get_arg(k).ok()).is_some_and(is_num);
                                let arg1_ok = outer_key.and_then(|k| engine.arena.get_arg(k).ok()).is_some_and(is_num);
                                arg0_ok && arg1_ok
                            }
                            _ => false,
                        }
                    } else {
                        false
                    };

                    if is_reducible && let Some(f) = callback {
                        f(EngineView::from_engine(engine));
                    }

                    prev_reduction = engine.reduce()?;
                }
            }
        }

        Ok(())
    }

    pub fn reduce(&self, input: &str) -> Result<String> {
        let root = parse_ski(input)?;

        let mut engine = Engine::from_tree(root);

        self.perform(&mut engine, &mut None::<fn(EngineView)>)?;

        Ok(format!("{}", EngineView::from_engine(&engine)))
    }

    pub fn reduce_with(&self, input: &str, mut callback: impl FnMut(EngineView)) -> Result<String> {
        let root = parse_ski(input)?;

        let mut engine = Engine::from_tree(root);

        self.perform(&mut engine, &mut Some(&mut callback))?;

        engine.spine.clear();
        callback(EngineView::from_engine(&engine));

        Ok(format!("{}", EngineView::from_engine(&engine)))
    }

    pub fn compute_with(&self, input: &str, mut callback: impl FnMut(EngineView)) -> Result<()> {
        let root = parse_ski(input)?;

        let mut engine = Engine::from_tree(root);

        self.perform(&mut engine, &mut Some(&mut callback))?;

        engine.spine.clear();
        callback(EngineView::from_engine(&engine));

        Ok(())
    }
}

impl fmt::Display for ReductionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NormalForm => "Normal form",
            Self::WeakHeadNormalForm => "Weak head normal form",
        })
    }
}
