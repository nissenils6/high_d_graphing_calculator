#[derive(Clone)]
pub enum Expr {
    Constant(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            &Expr::Constant(value) => value,
            Expr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expr::Sub(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expr::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
        }
    }

    pub fn add(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Add(Box::new(lhs), Box::new(rhs))
    }

    pub fn sub(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Sub(Box::new(lhs), Box::new(rhs))
    }

    pub fn mul(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Mul(Box::new(lhs), Box::new(rhs))
    }

    pub fn div(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Div(Box::new(lhs), Box::new(rhs))
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Expr::Constant(value) => write!(f, "{}", value),
            Expr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
        }
    }
}
