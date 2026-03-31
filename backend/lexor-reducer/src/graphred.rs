use core::{fmt::Debug, hash::Hash};

use lexor_parser::ski_parser::{ParsingError::ChumskyError, chumsky_parse as parse};
use thiserror::Error;

use crate::{
    core::{
        engine::{Engine, ReductionState},
        node::Node,
    },
    engineview::EngineView,
};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ReductionError {
    #[error("Arena error: {0}")]
    Arena(#[from] crate::core::arena::ArenaError),
    #[error("Engine error: {0}")]
    Engine(#[from] crate::core::engine::EngineError),
    #[error("Parsing error: {0}")]
    Parsing(#[from] lexor_parser::ski_parser::ParsingError),
}

type Result<T> = core::result::Result<T, ReductionError>;

/// A reduction strategy.
///
/// Note that this trait is sealed.
pub trait ReductionStrat:
    Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + super::seal::Sealed
{
    /// Performs the given reduction.
    ///
    /// # Errors
    ///
    /// When either the parser, the arena allocator or the reduction engine
    /// errors out.
    fn perform(engine: &mut Engine, callback: &mut Option<impl FnMut(EngineView)>) -> Result<()>;

    /// Reduces the given `input` based on the strategy used to invoke this
    /// function and returns the result as a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let result = NF::reduce("SKSKSSKSKSKSKSKSKSK").unwrap();
    ///
    /// assert_eq!(result, "K");
    /// ```
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn reduce(input: &str) -> Result<String> {
        let root = parse(input).map_err(|_| ChumskyError)?;

        let mut engine = Engine::from_tree(root);
        Self::perform(&mut engine, &mut None::<fn(EngineView)>)?;
        Ok(format!("{}", EngineView::from_engine(&engine)))
    }

    /// Reduces the given `input` based on the strategy ued to invoke this
    /// function and returns the result as a [`String`]. Executes `callback`
    /// after each reduction step.
    ///
    /// The type `Engine` implements the `Display` trait by returning
    /// the current reduction state. Thus you can view it as an "internal view".
    ///
    /// # Example
    ///
    /// ```
    /// use std::vec;
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let mut logs: Vec<String> = vec![];
    /// let mut count: u32 = 0;
    ///
    /// let result = NF::reduce_with("S(SKSKSSK)(SSKSKSK)I", |view| {
    ///                  count = count.saturating_add(1);
    ///                  logs.push(format!("Step #{count}: {view}\n"));
    ///              }).unwrap();
    ///
    /// assert_eq!(result, "SKI");
    /// assert_eq!(logs.len(), 10);
    /// assert_eq!(logs[0], "Step #1: SKSKSSKI(SSKSKSKI)\n");
    /// assert_eq!(logs[1], "Step #2: KK(SK)SSKI(SSKSKSKI)\n");
    /// assert_eq!(logs[2], "Step #3: KSSKI(SSKSKSKI)\n");
    /// assert_eq!(logs[3], "Step #4: SKI(SSKSKSKI)\n");
    /// assert_eq!(logs[4], "Step #5: K(SSKSKSKI)(I(SSKSKSKI))\n");
    /// assert_eq!(logs[5], "Step #6: SSKSKSKI\n");
    /// assert_eq!(logs[6], "Step #7: SS(KS)KSKI\n");
    /// assert_eq!(logs[7], "Step #8: SK(KSK)SKI\n");
    /// assert_eq!(logs[8], "Step #9: KS(KSKS)KI\n");
    /// assert_eq!(logs[9], "Step #10: SKI\n");
    /// ```
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn reduce_with(input: &str, mut callback: impl FnMut(EngineView)) -> Result<String> {
        let root = parse(input).map_err(|_| ChumskyError)?;
        let mut engine = Engine::from_tree(root);

        Self::perform(&mut engine, &mut Some(&mut callback))?;

        engine.spine.clear();
        callback(EngineView::from_engine(&engine));

        Ok(format!("{}", EngineView::from_engine(&engine)))
    }

    /// Computes the desired form of the given `input`.
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// // Internally, this reduces to "S"
    /// NF::compute("KSI");
    /// ```
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn compute(input: &str) -> Result<()> {
        let root = parse(input).map_err(|_| ChumskyError)?;

        let mut engine = Engine::from_tree(root);
        Self::perform(&mut engine, &mut None::<fn(EngineView)>)?;
        Ok(())
    }

    /// Computes the desired form of the given `input`. Executes `callback`
    /// after each reduction step.
    ///
    /// The type `Engine` implements the `Display` trait by returning
    /// the current reduction state. Thus you can view it as an "internal view".
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let mut count: u32 = 0;
    ///
    /// NF::compute_with("KSI", |_| { count = count.saturating_add(1); });
    ///
    /// assert_eq!(count, 1);
    /// ```
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn compute_with(input: &str, mut callback: impl FnMut(EngineView)) -> Result<()> {
        let root = parse(input).map_err(|_| ChumskyError)?;
        let mut engine = Engine::from_tree(root);

        Self::perform(&mut engine, &mut Some(&mut callback))?;

        engine.spine.clear();
        callback(EngineView::from_engine(&engine));

        Ok(())
    }
}

/// [Normal form] reduction strategy. Always reduces the current
/// leftmost-outermost redex to [`WeakHeadNormalForm`] until there are no more
/// saturated redexes in the whole term.
///
/// [Normal form]: https://en.wikipedia.org/wiki/Beta_normal_form#Normal_forms
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalForm;

impl Default for NormalForm {
    fn default() -> Self {
        Self
    }
}

/// A type alias for [`NormalForm`].
pub type NF = NormalForm;

impl ReductionStrat for NormalForm {
    fn perform(engine: &mut Engine, callback: &mut Option<impl FnMut(EngineView)>) -> Result<()> {
        let mut work_stack = Vec::with_capacity(20);
        work_stack.push(engine.subtree_root);

        while let Some(next_root) = work_stack.pop() {
            engine.subtree_root = next_root;
            engine.spine.clear();

            WHNF::perform(engine, callback)?;

            let _head = engine.spine.pop();
            for &app_node in &engine.spine {
                if let Ok(Node::App(_lhs, rhs)) = engine.arena.get(app_node) {
                    work_stack.push(*rhs);
                }
            }
        }

        Ok(())
    }
}

/// [Weak head normal form] reduction. Reduces until the head of the current
/// term isn't saturated (doesn't have enough arguments to reduce).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WeakHeadNormalForm;

impl Default for WeakHeadNormalForm {
    fn default() -> Self {
        Self
    }
}

/// A type alias for [`WeakHeadNormalForm`].
pub type WHNF = WeakHeadNormalForm;

impl ReductionStrat for WeakHeadNormalForm {
    fn perform(engine: &mut Engine, callback: &mut Option<impl FnMut(EngineView)>) -> Result<()> {
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
                if let Ok(Node::Comb(comb)) = engine.arena.get(head_key) {
                    engine.spine.len() > comb.arity()
                } else {
                    false
                }
            } else {
                false
            };

            if is_reducible && let Some(f) = callback {
                f(EngineView::from_engine(engine));
            }

            prev_reduction = engine.reduce()?;
        }

        Ok(())
    }
}

#[test]
fn playground() {
    let _ = NF::compute_with("SKSIKSISKS", |view| {
        println!("{view}");
    });
}
