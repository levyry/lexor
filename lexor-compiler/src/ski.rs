use std::{fmt, ops::BitAnd};

#[derive(Debug, Clone)]
pub enum Ski {
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

impl BitAnd for Ski {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::App(Box::new(self), Box::new(rhs))
    }
}

impl fmt::Display for Ski {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn pretty_print(body: &Ski) -> String {
            match body {
                Ski::S => String::from("S"),
                Ski::I => String::from("I"),
                Ski::C => String::from("C"),
                Ski::K => String::from("K"),
                Ski::B => String::from("B"),
                Ski::App(lhs, rhs) => {
                    if let Ski::App(_, _) = &**rhs {
                        format!("{lhs}({rhs})")
                    } else {
                        format!("{lhs}{rhs}")
                    }
                }
                Ski::Sn(n) => format!("S_{n}"),
                Ski::Bn(n) => format!("B_{n}"),
                Ski::Cn(n) => format!("C_{n}"),
            }
        }

        write!(f, "{}", pretty_print(self))
    }
}
