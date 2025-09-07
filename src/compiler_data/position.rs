//! The module of `Position`

use std::fmt::{Display, Formatter};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Position {
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl Position {
    #[must_use]
    pub fn combine(start: &Self, end: &Self) -> Self {
        Self {
            start_line: start.start_line,
            end_line: end.end_line,
            start_col: start.start_col,
            end_col: end.end_col,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line ")?;
        if self.start_line == self.end_line {
            write!(f, "{} | column ", self.start_line)?;
            if self.start_col == self.end_col {
                write!(f, "{}]", self.start_col)
            } else {
                write!(f, "{}:{}]", self.start_col, self.end_col)
            }
        } else {
            write!(f, "{}:{} | column", self.start_line, self.end_line)?;
            if self.start_col == self.end_col {
                write!(f, "{}]", self.start_col)
            } else {
                write!(f, "{}--{}]", self.start_col, self.end_col)
            }
        }
    }
}
