use std::fmt;

fn modular_pow(x: i64, exp: u32, modulo: i64) -> i64 {
    let pow = match x.checked_pow(exp) {
        Some(x) => x,
        None => {
            let exp_a = exp / 2;
            let exp_b = exp - exp_a;
            modular_pow(x, exp_a, modulo) * modular_pow(x, exp_b, modulo)
        }
    };

    pow % modulo
}

fn modular_multiplicative_inverse(a: i64, m: u32) -> i64 {
    modular_pow(a, m - 2, m as _)
}

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(var) = $target {
            var
        } else {
            panic!("Invalid variant casted to {}", stringify!($pat));
        }
    }};
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(i64),
    Var,
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
}

impl Expr {
    fn partition(items: &[Expr]) -> (Vec<Expr>, Vec<Expr>) {
        items
            .iter()
            .map(Self::reduce)
            .partition(|x| matches!(x, Self::Literal(_)))
    }
    fn reduce(&self) -> Expr {
        match self {
            &Expr::Literal(val) => Expr::Literal(val),
            Expr::Var => Expr::Var,
            Expr::Add(items) => {
                let (literals, mut others) = Expr::partition(items);
                if literals.is_empty() && others.is_empty() {
                    return Expr::Literal(0);
                }
                let sum = literals
                    .into_iter()
                    .map(|lit| cast!(lit, Expr::Literal))
                    .sum();
                if sum != 0 {
                    if others.is_empty() {
                        return Self::Literal(sum);
                    }
                    others.insert(0, Self::Literal(sum));
                }
                others.pop().unwrap_or(Expr::Add(others))
            }
            Expr::Mul(items) => {
                let (literals, mut others) = Expr::partition(items);
                if literals.is_empty() && others.is_empty() {
                    return Expr::Literal(1);
                }
                let product = literals
                    .into_iter()
                    .map(|lit| cast!(lit, Expr::Literal))
                    .product();
                if product != 1 {
                    if others.is_empty() {
                        return Self::Literal(product);
                    } else {
                        others.insert(0, Self::Literal(product));
                    }
                }
                others.pop().unwrap_or(Expr::Mul(others))
            }
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Var => write!(f, "x"),
            Expr::Add(terms) => {
                write!(f, "(")?;
                for (i, term) in terms.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{:?}", term)?;
                    } else {
                        write!(f, " + {:?}", term)?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
            Expr::Mul(terms) => {
                write!(f, "(")?;
                for (i, term) in terms.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{:?}", term)?;
                    } else {
                        write!(f, " * {:?}", term)?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinearCongruence {
    lhs: Expr,
    rhs: Expr,
    modulo: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(modular_multiplicative_inverse(13, 457), 211)
    }

    #[test]
    fn test_reduce() {
        assert_eq!(Expr::Add(vec![]).reduce(), Expr::Literal(0).reduce());

        assert_eq!(
            Expr::Add(vec![Expr::Literal(2), Expr::Literal(3)]).reduce(),
            Expr::Add(vec![Expr::Literal(5)]).reduce(),
        );

        assert_eq!(
            Expr::Add(vec![Expr::Literal(2), Expr::Literal(3), Expr::Literal(5)]).reduce(),
            Expr::Add(vec![Expr::Literal(10)]).reduce(),
        );

        assert_eq!(
            Expr::Add(vec![Expr::Literal(2), Expr::Literal(3), Expr::Var]).reduce(),
            Expr::Add(vec![Expr::Literal(5), Expr::Var]).reduce(),
        );

        assert_eq!(
            Expr::Mul(vec![Expr::Literal(2), Expr::Literal(3), Expr::Var]).reduce(),
            Expr::Mul(vec![Expr::Literal(6), Expr::Var]).reduce(),
        );

        assert_eq!(
            Expr::Mul(vec![
                Expr::Add(vec![Expr::Literal(2), Expr::Literal(3)]),
                Expr::Literal(10),
                Expr::Var
            ])
            .reduce(),
            Expr::Mul(vec![Expr::Literal(50), Expr::Var]).reduce(),
        );
    }
}
