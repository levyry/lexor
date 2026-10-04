use core::fmt;
use std::vec;

use serde::{Deserialize, Serialize};

use crate::lambda::Lambda;
use crate::literal::Literal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LambdaReductionStrategy {
    CallByName,
    CallByValue,
    NormalOrder,
}

impl fmt::Display for LambdaReductionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CallByName => "Call by name",
            Self::CallByValue => "Call by value",
            Self::NormalOrder => "Normal order",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeBruijn {
    BVar(usize),
    FVar(String),
    Abs(Box<Self>),
    App(Box<Self>, Box<Self>),
    Primitive(Literal),
}

#[allow(dead_code)]
impl DeBruijn {
    fn var_opening(self, index: usize, name: String) -> Self {
        self.beta_reduce(index, Self::FVar(name))
    }

    fn var_closing(self, name: String, index: usize) -> Self {
        match self {
            Self::BVar(_) | Self::Primitive(_) => self,
            Self::FVar(ref x) => {
                if *x == name {
                    Self::BVar(index)
                } else {
                    self
                }
            }
            Self::Abs(body) => Self::Abs(Box::new(body.var_closing(name, index.saturating_add(1)))),
            Self::App(lhs, rhs) => Self::App(
                Box::new(lhs.var_closing(name.clone(), index)),
                Box::new(rhs.var_closing(name, index)),
            ),
        }
    }

    fn is_closed_at(&self, limit: usize) -> bool {
        match self {
            Self::BVar(k) => *k < limit,
            Self::FVar(_) | Self::Primitive(_) => true,
            Self::Abs(body) => body.is_closed_at(limit.saturating_add(1)),
            Self::App(lhs, rhs) => lhs.is_closed_at(limit) && rhs.is_closed_at(limit),
        }
    }

    fn is_closed(&self) -> bool {
        self.is_closed_at(0)
    }

    fn collect_free(self, acc: Vec<Self>) -> Vec<String> {
        match self {
            Self::BVar(_) | Self::Primitive(_) => vec![],
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
            Self::BVar(_) | Self::Primitive(_) => self,
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
            Self::FVar(_) | Self::Primitive(_) => self,
            Self::Abs(body) => {
                Self::Abs(Box::new(body.beta_reduce(index.saturating_add(1), other)))
            }
            Self::App(lhs, rhs) => Self::App(
                Box::new(lhs.beta_reduce(index, other.clone())),
                Box::new(rhs.beta_reduce(index, other)),
            ),
        }
    }

    #[must_use]
    pub fn evaluate(self, strat: LambdaReductionStrategy) -> Self {
        match strat {
            LambdaReductionStrategy::CallByName => {
                if let Self::App(lhs, rhs) = self {
                    let lhs_eval = lhs.evaluate(strat);
                    if let Self::Abs(body) = lhs_eval {
                        body.beta_reduce(0, *rhs).evaluate(strat)
                    } else {
                        Self::App(Box::new(lhs_eval), rhs)
                    }
                } else {
                    self
                }
            }
            LambdaReductionStrategy::CallByValue => {
                if let Self::App(lhs, rhs) = self {
                    let lhs_eval = lhs.evaluate(strat);
                    let rhs_eval = rhs.evaluate(strat);
                    if let Self::Abs(body) = lhs_eval {
                        body.beta_reduce(0, rhs_eval).evaluate(strat)
                    } else {
                        Self::App(Box::new(lhs_eval), Box::new(rhs_eval))
                    }
                } else {
                    self
                }
            }
            LambdaReductionStrategy::NormalOrder => {
                if let Self::Abs(body) = self {
                    Self::Abs(Box::new(body.evaluate(strat)))
                } else if let Self::App(lhs, rhs) = self {
                    let lhs_whnf = lhs.clone().evaluate(LambdaReductionStrategy::CallByName);
                    if let Self::Abs(body) = lhs_whnf {
                        body.beta_reduce(0, *rhs).evaluate(strat)
                    } else {
                        Self::App(Box::new(lhs.evaluate(strat)), Box::new(rhs.evaluate(strat)))
                    }
                } else {
                    self
                }
            }
        }
    }
}

impl From<Lambda> for DeBruijn {
    fn from(expr: Lambda) -> Self {
        fn convert(expr: Lambda, scope: &mut Vec<String>) -> DeBruijn {
            match expr {
                Lambda::Var(name) => scope
                    .iter()
                    .rev()
                    .position(|n| n == &name)
                    .map_or(DeBruijn::FVar(name), DeBruijn::BVar),
                Lambda::Abs(arg, body) => {
                    scope.push(arg);
                    let body_de_bruijn = convert(*body, scope);
                    scope.pop();
                    DeBruijn::Abs(Box::new(body_de_bruijn))
                }
                Lambda::App(lhs, rhs) => DeBruijn::App(
                    Box::new(convert(*lhs, scope)),
                    Box::new(convert(*rhs, scope)),
                ),
                Lambda::Primitive(p) => DeBruijn::Primitive(p),
            }
        }
        convert(expr, &mut Vec::new())
    }
}
