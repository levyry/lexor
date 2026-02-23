/*!
This module provides a definition for a Lambda calculus term in the
(locally nameless) De Bruijn representation.

TODO: Finish docs, add example
*/

use std::vec;

use lower::saturating::math as saturating;

/// Reduction strategies. TODO: Write this doc.
// TODO: Refactor this to work similarly to how we handle
//       combinator reduction kinds (trait based)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReductionStrategy {
    CallByName,
    CallByValue,
    NormalOrder,
}

/// Locally nameless De Bruijn representation.
///
/// Based on: <https://chargueraud.org/research/2009/ln/main.pdf>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeBruijn {
    BVar(usize),
    FVar(String),
    Abs(Box<Self>),
    App(Box<Self>, Box<Self>),
}

// TODO: Remove once I figure out if these methods are really needed or not.
#[allow(dead_code)]
impl DeBruijn {
    fn var_opening(self, index: usize, name: String) -> Self {
        self.beta_reduce(index, Self::FVar(name))
    }

    fn var_closing(self, name: String, index: usize) -> Self {
        match self {
            Self::BVar(_) => self,

            Self::FVar(ref x) => {
                if *x == name {
                    Self::BVar(index)
                } else {
                    self
                }
            }

            Self::Abs(body) => Self::Abs(Box::new(body.var_closing(name, saturating!(index + 1)))),

            Self::App(lhs, rhs) => Self::App(
                Box::new(lhs.var_closing(name.clone(), index)),
                Box::new(rhs.var_closing(name, index)),
            ),
        }
    }

    fn is_closed_at(&self, limit: usize) -> bool {
        match self {
            Self::BVar(k) => *k < limit,
            Self::FVar(_) => true,
            Self::Abs(body) => body.is_closed_at(limit.saturating_add(1)),
            Self::App(lhs, rhs) => lhs.is_closed_at(limit) && rhs.is_closed_at(limit),
        }
    }

    fn is_closed(&self) -> bool {
        self.is_closed_at(0)
    }

    fn collect_free(self, acc: Vec<Self>) -> Vec<String> {
        match self {
            Self::BVar(_) => vec![],
            Self::FVar(name) => vec![name],
            Self::Abs(body) => body.collect_free(acc),
            Self::App(lhs, rhs) => {
                let new_lhs = lhs.collect_free(acc.clone());
                let new_rhs = rhs.collect_free(acc);

                let mut result: Vec<String> = vec![];

                for var in &new_lhs {
                    result.push(var.to_owned());
                }

                for var in &new_rhs {
                    result.push(var.to_owned());
                }

                result
            }
        }
    }

    fn substitute(self, original: String, replacement: &Self) -> Self {
        match self {
            Self::BVar(_) => self,

            Self::FVar(ref name) => {
                if *name == original {
                    replacement.clone()
                } else {
                    self
                }
            }

            Self::Abs(body) => Self::Abs(Box::new(body.substitute(original, replacement))),

            Self::App(lhs, rhs) => Self::App(
                Box::new(lhs.substitute(original.clone(), &replacement.clone())),
                Box::new(rhs.substitute(original, &replacement.clone())),
            ),
        }
    }

    fn beta_reduce(self, index: usize, other: Self) -> Self {
        match self {
            Self::BVar(k) => {
                if k == index {
                    other
                } else {
                    self
                }
            }

            Self::FVar(_) => self,

            Self::Abs(body) => Self::Abs(Box::new(body.beta_reduce(saturating!(index + 1), other))),

            Self::App(lhs, rhs) => Self::App(
                Box::new(lhs.beta_reduce(index, other.clone())),
                Box::new(rhs.beta_reduce(index, other)),
            ),
        }
    }

    #[must_use]
    pub fn evaluate(self, strat: ReductionStrategy) -> Self {
        match strat {
            ReductionStrategy::CallByName => match self {
                Self::App(lhs, rhs) => {
                    let lhs_eval = lhs.evaluate(strat);

                    if let Self::Abs(body) = lhs_eval {
                        body.beta_reduce(0, *rhs).evaluate(strat)
                    } else {
                        Self::App(Box::new(lhs_eval), rhs)
                    }
                }
                _ => self,
            },
            ReductionStrategy::CallByValue => match self {
                Self::App(lhs, rhs) => {
                    let lhs_eval = lhs.evaluate(strat);
                    let rhs_eval = rhs.evaluate(strat);

                    if let Self::Abs(body) = lhs_eval {
                        body.beta_reduce(0, rhs_eval).evaluate(strat)
                    } else {
                        Self::App(Box::new(lhs_eval), Box::new(rhs_eval))
                    }
                }
                _ => self,
            },
            ReductionStrategy::NormalOrder => match self {
                Self::Abs(body) => Self::Abs(Box::new(body.evaluate(strat))),

                Self::App(lhs, rhs) => {
                    let lhs_whnf = lhs.clone().evaluate(ReductionStrategy::CallByName);

                    if let Self::Abs(body) = lhs_whnf {
                        body.beta_reduce(0, *rhs).evaluate(strat)
                    } else {
                        Self::App(Box::new(lhs.evaluate(strat)), Box::new(rhs.evaluate(strat)))
                    }
                }
                _ => self,
            },
        }
    }
}
