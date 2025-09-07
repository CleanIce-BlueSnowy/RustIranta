//! The module of `BinaryOperator`

use std::fmt::Display;

pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Multiply => "*",
                Self::Divide => "/",
                Self::Power => "**",
            }
        )
    }
}
