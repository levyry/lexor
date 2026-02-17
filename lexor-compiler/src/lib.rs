#![allow(unused)]

use std::{
    cmp::Ordering,
    fmt::{self},
    ops::BitAnd,
};

// data Deb = Zero | Succ Deb | Lam Deb | App Deb Deb deriving Show
#[derive(Debug, Clone)]
enum Deb {
    Zero,
    Succ(Box<Self>),
    Lam(Box<Self>),
    App(Box<Self>, Box<Self>),
}

// data Com = Com :# Com | S | I | C | K | B | Sn Int | Bn Int | Cn Int
#[derive(Debug, Clone)]
enum Com {
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
        fn pretty_print(body: &Com) -> String {
            match body {
                Com::S => String::from("S"),
                Com::I => String::from("I"),
                Com::C => String::from("C"),
                Com::K => String::from("K"),
                Com::B => String::from("B"),
                Com::App(lhs, rhs) => {
                    if let Com::App(_, _) = &**rhs {
                        format!("{lhs}({rhs})")
                    } else {
                        format!("{lhs}{rhs}")
                    }
                }
                Com::Sn(n) => format!("S_{n}"),
                Com::Bn(n) => format!("B_{n}"),
                Com::Cn(n) => format!("C_{n}"),
            }
        }

        write!(f, "{}", pretty_print(self))
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
fn ski(deb: Deb) -> (usize, Com) {
    //   where
    //   f (a, x) (b, y) = case (a, b) of
    //     (0, 0)             ->         x :# y
    //     (0, n)             -> Bn n :# x :# y
    //     (n, 0)             -> Cn n :# x :# y
    //     (n, m) | n == m    -> Sn n :# x :# y
    //            | n < m     ->                Bn (m - n) :# (Sn n :# x) :# y
    //            | otherwise -> Cn (n - m) :# (Bn (n - m) :#  Sn m :# x) :# y
    fn f(ax: (usize, Com), by: (usize, Com)) -> Com {
        let (lhs_num, lhs_body) = ax;
        let (rhs_num, rhs_body) = by;

        match (lhs_num, rhs_num) {
            (0, 0) => lhs_body & rhs_body,
            (0, n) => Com::Bn(n) & lhs_body & rhs_body,
            (n, 0) => Com::Cn(n) & lhs_body & rhs_body,
            (n, m) => match n.cmp(&m) {
                Ordering::Equal => Com::Sn(n) & lhs_body & rhs_body,
                Ordering::Less => Com::Bn(m.saturating_sub(n)) & (Com::Sn(n) & lhs_body) & rhs_body,
                Ordering::Greater => {
                    Com::Cn(n.saturating_sub(m))
                        & (Com::Bn(n.saturating_sub(m)) & Com::Sn(m) & lhs_body)
                        & rhs_body
                }
            },
        }
    }

    match deb {
        Deb::Zero => (1, Com::I),
        Deb::Succ(body) => {
            let (n, inner) = ski(*body);
            (n.saturating_add(1), f((0, Com::K), (n, inner)))
        }
        Deb::App(lhs, rhs) => {
            let (a, lhs_body) = ski(*lhs);
            let (b, rhs_body) = ski(*rhs);
            (usize::max(a, b), f((a, lhs_body), (b, rhs_body)))
        }
        Deb::Lam(body) => {
            let (n, inner) = ski(*body);
            if n == 0 {
                (0, Com::K & inner)
            } else {
                (n.saturating_sub(1), inner)
            }
        }
    }
}

// linBulk :: Com -> Com
// linBulk b = case b of
//   Bn n   -> iterate ((B:#        B):#) B !! (n - 1)
//   Cn n   -> iterate ((B:#(B:#C):#B):#) C !! (n - 1)
//   Sn n   -> iterate ((B:#(B:#S):#B):#) S !! (n - 1)
//   x :# y -> linBulk x :# linBulk y
//   _      -> b
#[allow(clippy::eq_op)]
fn lin_bulk(b: Com) -> Com {
    match b {
        Com::Bn(n) => {
            let mut acc = Com::B;
            for _ in 0..n.saturating_sub(1) {
                acc = (Com::B & Com::B) & acc;
            }
            acc
        }
        Com::Cn(n) => {
            let mut acc = Com::C;
            for _ in 0..n.saturating_sub(1) {
                acc = (Com::B & (Com::B & Com::C) & Com::B) & acc;
            }
            acc
        }
        Com::Sn(n) => {
            let mut acc = Com::S;
            for _ in 0..n.saturating_sub(1) {
                acc = (Com::B & (Com::B & Com::S) & Com::B) & acc;
            }
            acc
        }
        Com::App(lhs, rhs) => lin_bulk(*lhs) & lin_bulk(*rhs),
        _ => b,
    }
}

// logBulk :: Com -> Com
// logBulk b = case b of
//   Bn n   -> go n (K:#I)         :# B              :# I
//   Cn n   -> go n (K:#(C:#I:#I)) :# (B:#(B:#C):#B) :# I
//   Sn n   -> go n (K:#(C:#I:#I)) :# (B:#(B:#S):#B) :# I
//   x :# y -> logBulk x :# logBulk y
//   _      -> b
fn log_bulk(b: Com) -> Com {
    fn get_bits(mut n: usize) -> Vec<usize> {
        let mut acc = Vec::new();
        if n == 0 {
            return acc;
        }
        while n > 0 {
            let (q, r) = (n / 2, n % 2);
            acc.push(r);
            n = q;
        }
        acc.reverse();
        acc
    }

    fn b0() -> Com {
        Com::C & Com::B & (Com::S & Com::B & Com::I)
    }

    #[allow(clippy::eq_op)]
    fn b1() -> Com {
        Com::C
            & (Com::B
                & Com::S
                & (Com::B & (Com::B & Com::B) & (Com::C & Com::B & (Com::S & Com::B & Com::I))))
            & Com::B
    }

    //   where
    //   go n base = foldr (:#) base $ ([b0, b1]!!) <$> bits [] n
    //   bits acc 0 = reverse acc
    //   bits acc n | (q, r) <- divMod n 2 = bits (r:acc) q
    //   b0 = C:#B:#(S:#B:#I)
    //   b1 = C:#(B:#S:#(B:#(B:#B):#(C:#B:#(S:#B:#I)))):#B
    fn go(n: usize, base: Com) -> Com {
        let bits = get_bits(n);
        let mut acc = base;

        for bit in bits.iter().rev() {
            let op = if *bit == 0 { b0() } else { b1() };
            acc = op & acc;
        }
        acc
    }

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
