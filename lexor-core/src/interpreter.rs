#![allow(unused)]

use crate::de_bruijn::DeBruijn;

trait Interpreter {
    type Term;

    fn eval(term: Self::Term) -> Self::Term;
}

pub struct CallByValue;
impl Interpreter for CallByValue {
    type Term = DeBruijn;

    fn eval(term: Self::Term) -> Self::Term {
        match term {
            DeBruijn::App(lhs, rhs) => {
                let lhs_eval = Self::eval(*lhs);
                let rhs_eval = Self::eval(*rhs);

                if let DeBruijn::Abs(body) = lhs_eval {
                    Self::eval(body.beta_reduce(0, rhs_eval))
                } else {
                    DeBruijn::App(Box::new(lhs_eval), Box::new(rhs_eval))
                }
            }
            _ => term,
        }
    }
}

pub struct CallByName;
impl Interpreter for CallByName {
    type Term = DeBruijn;

    fn eval(term: Self::Term) -> Self::Term {
        match term {
            DeBruijn::App(lhs, rhs) => {
                let lhs_eval = Self::eval(*lhs);

                if let DeBruijn::Abs(body) = lhs_eval {
                    Self::eval(body.beta_reduce(0, *rhs))
                } else {
                    DeBruijn::App(Box::new(lhs_eval), rhs)
                }
            }
            _ => term,
        }
    }
}

pub struct NormalOrder;
impl Interpreter for NormalOrder {
    type Term = DeBruijn;

    fn eval(term: Self::Term) -> Self::Term {
        match term {
            DeBruijn::Abs(body) => DeBruijn::Abs(Box::new(Self::eval(*body))),

            DeBruijn::App(lhs, rhs) => {
                let lhs_whnf = CallByName::eval(*lhs.clone());

                if let DeBruijn::Abs(body) = lhs_whnf {
                    Self::eval(body.beta_reduce(0, *rhs))
                } else {
                    DeBruijn::App(Box::new(Self::eval(*lhs)), Box::new(Self::eval(*rhs)))
                }
            }
            _ => term,
        }
    }
}
