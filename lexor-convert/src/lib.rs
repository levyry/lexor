#![allow(unused)]

#![allow(clippy::eq_op)]

use std::{
    cmp::Ordering,
    fmt::{self},
    ops::BitAnd,
    sync::LazyLock,
};

#[derive(Debug, Clone)]
enum Deb {
    Zero,
    Succ(Box<Self>),
    Lam(Box<Self>),
    App(Box<Self>, Box<Self>),
}

#[derive(Debug, Clone)]
pub enum Com {
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

// infixl 5 :#
impl BitAnd for Com {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::App(Box::new(self), Box::new(rhs))
    }
}

// instance Show Com where
//   show S = "S"
//   show I = "I"
//   show C = "C"
//   show K = "K"
//   show B = "B"
//   show (l :# r@(_ :# _)) = show l ++ "(" ++ show r ++ ")"
//   show (l :# r)          = show l ++        show r
//   show (Bn n) = "B_" ++ show n
//   show (Cn n) = "C_" ++ show n
//   show (Sn n) = "S_" ++ show n
impl fmt::Display for Com {
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

// ski :: Deb -> (Int, Com)
// ski deb = case deb of
//   Zero                           -> (1,       I)
//   Succ d    | x@(n, _) <- ski d  -> (n + 1,   f (0, K) x)
//   App d1 d2 | x@(a, _) <- ski d1
//             , y@(b, _) <- ski d2 -> (max a b, f x y)
//   Lam d | (n, e) <- ski d -> case n of
//                                0 -> (0,       K :# e)
//                                _ -> (n - 1,   e)
fn convert(deb: Deb) -> (usize, Com) {
    match deb {
        Deb::Zero => (1, Com::I),
        Deb::Succ(body) => {
            let (n, inner) = convert(*body);
            (n.saturating_add(1), apply((0, Com::K), (n, inner)))
        }
        Deb::App(lhs, rhs) => {
            let (a, lhs_body) = convert(*lhs);
            let (b, rhs_body) = convert(*rhs);
            (usize::max(a, b), apply((a, lhs_body), (b, rhs_body)))
        }
        Deb::Lam(body) => {
            let (n, inner) = convert(*body);
            if n == 0 {
                (0, Com::K & inner)
            } else {
                (n.saturating_sub(1), inner)
            }
        }
    }
}

fn apply(ax: (usize, Com), by: (usize, Com)) -> Com {
    let (lhs_num, lhs_body) = ax;
    let (rhs_num, rhs_body) = by;

    match (lhs_num, rhs_num) {
        (0, 0) => lhs_body & rhs_body,
        (0, n) => Com::Bn(n) & lhs_body & rhs_body,
        (n, 0) => Com::Cn(n) & lhs_body & rhs_body,
        (n, m) => {
            let diff = n.abs_diff(m);

            let lhs_part = match n.cmp(&m) {
                Ordering::Equal => Com::Sn(n) & lhs_body,
                Ordering::Less => Com::Bn(diff) & (Com::Sn(n) & lhs_body),
                Ordering::Greater => Com::Cn(diff) & (Com::Bn(diff) & Com::Sn(m) & lhs_body),
            };

            lhs_part & rhs_body
        }
    }
}

#[must_use]
pub fn lin_bulk(b: Com) -> Com {
    let mut acc: Com;

    match b {
        Com::Bn(n) => {
            acc = Com::B;
            for _ in 1..n {
                acc = (Com::B & Com::B) & acc;
            }
        }
        Com::Cn(n) => {
            acc = Com::C;
            for _ in 1..n {
                acc = (Com::B & (Com::B & Com::C) & Com::B) & acc;
            }
        }
        Com::Sn(n) => {
            acc = Com::S;
            for _ in 1..n {
                acc = (Com::B & (Com::B & Com::S) & Com::B) & acc;
            }
        }
        Com::App(lhs, rhs) => return lin_bulk(*lhs) & lin_bulk(*rhs),
        _ => return b,
    }

    acc
}

fn log_bulk(b: Com) -> Com {
    match b {
        Com::Bn(n) => go(n, Com::K & Com::I) & Com::B & Com::I,
        Com::Cn(n) => {
            go(n, Com::K & (Com::C & Com::I & Com::I))
                & (Com::B & (Com::B & Com::C) & Com::B)
                & Com::I
        }
        Com::Sn(n) => {
            go(n, Com::K & (Com::C & Com::I & Com::I))
                & (Com::B & (Com::B & Com::S) & Com::B)
                & Com::I
        }
        Com::App(lhs, rhs) => log_bulk(*lhs) & log_bulk(*rhs),
        _ => b,
    }
}

static B0: LazyLock<Com> = LazyLock::new(|| Com::C & Com::B & (Com::S & Com::B & Com::I));

static B1: LazyLock<Com> = LazyLock::new(|| {
    Com::C
        & (Com::B
            & Com::S
            & (Com::B & (Com::B & Com::B) & (Com::C & Com::B & (Com::S & Com::B & Com::I))))
        & Com::B
});

fn go(n: usize, base: Com) -> Com {
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
        // \a b c d.d c b a
        // let lambda_term =
        // println!("{}", );
    }

    #[test]
    fn log_bulk_test() {
        let x = log_bulk(Com::Sn(1234));
        let y = log_bulk(Com::Bn(1234));

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
        let x = lin_bulk(Com::Sn(12));
        let y = lin_bulk(Com::Bn(11));
        let z = lin_bulk(Com::Cn(13));

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
