//! The module of `UnaryOperator`

use std::fmt::Display;

pub enum UnaryOperator {
    Plus,
    Minus,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
            }
        )
    }
}
