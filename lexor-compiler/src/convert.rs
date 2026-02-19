use crate::lir::Lir;
use crate::ski::Ski;
use std::cmp::Ordering;

fn convert(deb: Lir) -> (usize, Ski) {
    match deb {
        Lir::Zero => (1, Ski::I),
        Lir::Succ(body) => {
            let (n, inner) = convert(*body);
            (n.saturating_add(1), apply_bulk((0, Ski::K), (n, inner)))
        }
        Lir::App(lhs, rhs) => {
            let (a, lhs_body) = convert(*lhs);
            let (b, rhs_body) = convert(*rhs);
            (usize::max(a, b), apply_bulk((a, lhs_body), (b, rhs_body)))
        }
        Lir::Lam(body) => {
            let (n, inner) = convert(*body);
            if n == 0 {
                (0, Ski::K & inner)
            } else {
                (n.saturating_sub(1), inner)
            }
        }
    }
}

fn apply_bulk(ax: (usize, Ski), by: (usize, Ski)) -> Ski {
    let (lhs_num, lhs_body) = ax;
    let (rhs_num, rhs_body) = by;

    match (lhs_num, rhs_num) {
        (0, 0) => lhs_body & rhs_body,
        (0, n) => Ski::Bn(n) & lhs_body & rhs_body,
        (n, 0) => Ski::Cn(n) & lhs_body & rhs_body,
        (n, m) => match n.cmp(&m) {
            Ordering::Equal => Ski::Sn(n) & lhs_body & rhs_body,
            Ordering::Less => Ski::Bn(m.saturating_sub(n)) & (Ski::Sn(n) & lhs_body) & rhs_body,
            Ordering::Greater => {
                Ski::Cn(n.saturating_sub(m))
                    & (Ski::Bn(n.saturating_sub(m)) & Ski::Sn(m) & lhs_body)
                    & rhs_body
            }
        },
    }
}

#[allow(clippy::eq_op)]
fn lin_bulk(b: Ski) -> Ski {
    match b {
        Ski::Bn(n) => {
            let mut acc = Ski::B;
            for _ in 0..n.saturating_sub(1) {
                acc = (Ski::B & Ski::B) & acc;
            }
            acc
        }
        Ski::Cn(n) => {
            let mut acc = Ski::C;
            for _ in 0..n.saturating_sub(1) {
                acc = (Ski::B & (Ski::B & Ski::C) & Ski::B) & acc;
            }
            acc
        }
        Ski::Sn(n) => {
            let mut acc = Ski::S;
            for _ in 0..n.saturating_sub(1) {
                acc = (Ski::B & (Ski::B & Ski::S) & Ski::B) & acc;
            }
            acc
        }
        Ski::App(lhs, rhs) => lin_bulk(*lhs) & lin_bulk(*rhs),
        _ => b,
    }
}

fn log_bulk(b: Ski) -> Ski {
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

    fn b0() -> Ski {
        Ski::C & Ski::B & (Ski::S & Ski::B & Ski::I)
    }

    #[allow(clippy::eq_op)]
    fn b1() -> Ski {
        Ski::C
            & (Ski::B
                & Ski::S
                & (Ski::B & (Ski::B & Ski::B) & (Ski::C & Ski::B & (Ski::S & Ski::B & Ski::I))))
            & Ski::B
    }

    fn iterate(n: usize, base: Ski) -> Ski {
        let bits = get_bits(n);
        let mut acc = base;

        for bit in bits.iter().rev() {
            let op = if *bit == 0 { b0() } else { b1() };
            acc = op & acc;
        }
        acc
    }

    match b {
        Ski::Bn(n) => iterate(n, Ski::K & Ski::I) & Ski::B & Ski::I,

        Ski::Cn(n) => {
            iterate(n, Ski::K & (Ski::C & Ski::I & Ski::I))
                & (Ski::B & (Ski::B & Ski::C) & Ski::B)
                & Ski::I
        }

        Ski::Sn(n) => {
            iterate(n, Ski::K & (Ski::C & Ski::I & Ski::I))
                & (Ski::B & (Ski::B & Ski::S) & Ski::B)
                & Ski::I
        }

        Ski::App(lhs, rhs) => log_bulk(*lhs) & log_bulk(*rhs),

        _ => b,
    }
}
