//! This module implements the time-linear and space-linear bracket abstraction
//! algorithm found by Oleg Kiselyov.
#![allow(clippy::eq_op)]

use crate::{combinator::Combinator, de_bruijn::DeBruijn};

use std::{
    cmp::Ordering,
    fmt::{self},
    ops::BitAnd,
    sync::LazyLock,
};

#[must_use]
pub fn kiselyov(deb: &DeBruijn, mode: &BulkResolver) -> (usize, Combinator) {
    let (n, bulk) = convert(deb);

    let result = if *mode == BulkResolver::Linear {
        lin_bulk(&bulk)
    } else {
        log_bulk(&bulk)
    };

    (n, result)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BulkResolver {
    Linear,
    Logarithmic,
}

// Internal combinator representation that supports bulk combinators
#[derive(Debug, Clone)]
pub enum BulkCom {
    App(Box<Self>, Box<Self>),
    S,
    I,
    C,
    K,
    B,
    Sn(usize),
    Bn(usize),
    Cn(usize),
}

// Application operator for ease of describing complex terms
impl BitAnd for BulkCom {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::App(Box::new(self), Box::new(rhs))
    }
}

impl fmt::Display for BulkCom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Self::S => String::from("S"),
            Self::I => String::from("I"),
            Self::C => String::from("C"),
            Self::K => String::from("K"),
            Self::B => String::from("B"),
            Self::App(lhs, rhs) => {
                if let Self::App(_, _) = **rhs {
                    format!("{lhs}({rhs})")
                } else {
                    format!("{lhs}{rhs}")
                }
            }
            Self::Sn(n) => format!("S_{n}"),
            Self::Bn(n) => format!("B_{n}"),
            Self::Cn(n) => format!("C_{n}"),
        };

        write!(f, "{result}")
    }
}

pub(crate) fn convert(deb: &DeBruijn) -> (usize, BulkCom) {
    match deb {
        DeBruijn::BVar(0) => (1, BulkCom::I),
        DeBruijn::BVar(n) => {
            let mut inner_acc = BulkCom::I;
            let n1 = n.saturating_add(1);
            for i in 1..n1 {
                inner_acc = apply((0, BulkCom::K), (i, inner_acc));
            }
            (n1, inner_acc)
        }
        DeBruijn::App(lhs, rhs) => {
            let (a, lhs_body) = convert(lhs);
            let (b, rhs_body) = convert(rhs);
            (usize::max(a, b), apply((a, lhs_body), (b, rhs_body)))
        }
        DeBruijn::Abs(body) => {
            let (n, inner) = convert(body);
            if n == 0 {
                (0, BulkCom::K & inner)
            } else {
                (n.saturating_sub(1), inner)
            }
        }
        DeBruijn::FVar(_) => todo!(),
    }
}

fn apply((lhs_num, lhs_body): (usize, BulkCom), (rhs_num, rhs_body): (usize, BulkCom)) -> BulkCom {
    match (lhs_num, rhs_num) {
        (0, 0) => lhs_body & rhs_body,
        (0, n) => BulkCom::Bn(n) & lhs_body & rhs_body,
        (n, 0) => BulkCom::Cn(n) & lhs_body & rhs_body,
        (n, m) => {
            let diff = n.abs_diff(m);

            let lhs_part = match n.cmp(&m) {
                Ordering::Equal => BulkCom::Sn(n) & lhs_body,
                Ordering::Less => BulkCom::Bn(diff) & (BulkCom::Sn(n) & lhs_body),
                Ordering::Greater => {
                    BulkCom::Cn(diff) & (BulkCom::Bn(diff) & BulkCom::Sn(m) & lhs_body)
                }
            };

            lhs_part & rhs_body
        }
    }
}

#[must_use]
pub(crate) fn lin_bulk(bulk: &BulkCom) -> Combinator {
    let mut result: Combinator;

    match *bulk {
        BulkCom::Bn(n) => {
            result = Combinator::B;
            for _ in 1..n {
                result = (Combinator::B & Combinator::B) & result;
            }
        }
        BulkCom::Cn(n) => {
            result = Combinator::C;
            for _ in 1..n {
                result = (Combinator::B & (Combinator::B & Combinator::C) & Combinator::B) & result;
            }
        }
        BulkCom::Sn(n) => {
            result = Combinator::S;
            for _ in 1..n {
                result = (Combinator::B & (Combinator::B & Combinator::S) & Combinator::B) & result;
            }
        }
        BulkCom::App(ref lhs, ref rhs) => result = lin_bulk(lhs) & lin_bulk(rhs),
        BulkCom::S => result = Combinator::S,
        BulkCom::I => result = Combinator::I,
        BulkCom::C => result = Combinator::C,
        BulkCom::K => result = Combinator::K,
        BulkCom::B => result = Combinator::B,
    }

    result
}

pub(crate) fn log_bulk(bulk: &BulkCom) -> Combinator {
    match *bulk {
        BulkCom::Bn(n) => go(n, Combinator::K & Combinator::I) & Combinator::B & Combinator::I,
        BulkCom::Cn(n) => {
            go(
                n,
                Combinator::K & (Combinator::C & Combinator::I & Combinator::I),
            ) & (Combinator::B & (Combinator::B & Combinator::C) & Combinator::B)
                & Combinator::I
        }
        BulkCom::Sn(n) => {
            go(
                n,
                Combinator::K & (Combinator::C & Combinator::I & Combinator::I),
            ) & (Combinator::B & (Combinator::B & Combinator::S) & Combinator::B)
                & Combinator::I
        }
        BulkCom::App(ref lhs, ref rhs) => log_bulk(lhs) & log_bulk(rhs),
        BulkCom::S => Combinator::S,
        BulkCom::I => Combinator::I,
        BulkCom::C => Combinator::C,
        BulkCom::K => Combinator::K,
        BulkCom::B => Combinator::B,
    }
}

static B0: LazyLock<Combinator> = LazyLock::new(|| {
    Combinator::C & Combinator::B & (Combinator::S & Combinator::B & Combinator::I)
});

static B1: LazyLock<Combinator> = LazyLock::new(|| {
    Combinator::C
        & (Combinator::B
            & Combinator::S
            & (Combinator::B
                & (Combinator::B & Combinator::B)
                & (Combinator::C
                    & Combinator::B
                    & (Combinator::S & Combinator::B & Combinator::I))))
        & Combinator::B
});

fn go(n: usize, base: Combinator) -> Combinator {
    let num_bits = if n == 0 {
        0
    } else {
        usize::BITS.saturating_sub(n.leading_zeros())
    };

    (0..num_bits)
        .map(|i| {
            if (n >> i) & 1 == 1 {
                B1.clone()
            } else {
                B0.clone()
            }
        })
        .rfold(base, |acc, elem| elem & acc)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn playground() {
        println!("{}", log_bulk(&BulkCom::Sn(50)));
    }

    #[test]
    fn log_bulk_test() {
        let x = log_bulk(&BulkCom::Sn(1234));
        let y = log_bulk(&BulkCom::Bn(1234));

        assert_eq!(
            format!("{x}"),
            "CB(SBI)(C(BS(B(BB)(CB(SBI))))B(CB(SBI)(CB(SBI)(C(BS(B(BB)(CB(SBI))))B(CB(SBI)(C(BS(B(BB)(CB(SBI))))B(C(BS(B(BB)(CB(SBI))))B(CB(SBI)(CB(SBI)(C(BS(B(BB)(CB(SBI))))B(K(CII))))))))))))(B(BS)B)I",
            "Sn log_bulk failed"
        );
        assert_eq!(
            format!("{y}"),
            "CB(SBI)(C(BS(B(BB)(CB(SBI))))B(CB(SBI)(CB(SBI)(C(BS(B(BB)(CB(SBI))))B(CB(SBI)(C(BS(B(BB)(CB(SBI))))B(C(BS(B(BB)(CB(SBI))))B(CB(SBI)(CB(SBI)(C(BS(B(BB)(CB(SBI))))B(KI)))))))))))BI",
            "Bn log_bulk failed"
        );
    }

    #[test]
    fn lin_bulk_test() {
        let x = lin_bulk(&BulkCom::Sn(12));
        let y = lin_bulk(&BulkCom::Bn(11));
        let z = lin_bulk(&BulkCom::Cn(13));

        assert_eq!(
            format!("{x}"),
            "B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)B(B(BS)BS))))))))))",
            "Sn lin_bulk failed"
        );
        assert_eq!(
            format!("{y}"),
            "BB(BB(BB(BB(BB(BB(BB(BB(BB(BBB)))))))))",
            "Bn lin_bulk failed"
        );
        assert_eq!(
            format!("{z}"),
            "B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)B(B(BC)BC)))))))))))",
            "Cn lin_bulk failed"
        );
    }
}
