#[derive(Debug, Clone)]
pub enum Lir {
    Zero,
    Succ(Box<Self>),
    Lam(Box<Self>),
    App(Box<Self>, Box<Self>),
}
