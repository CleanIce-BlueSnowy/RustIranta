//! The module for `SyntaxError`

use crate::compiler_data::position::Position;
use crate::lexer::error::LexicalError;
use crate::main_error::IrantaCompilerError;

pub struct SyntaxError {
    pub pos: Position,
    pub msg: String,
}

pub struct SyntaxErrorList {
    pub list: Vec<SyntaxError>,
}

impl IrantaCompilerError for SyntaxError {
    fn get_pos(&self) -> &Position {
        &self.pos
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }
}

impl SyntaxError {
    #[must_use]
    pub fn new(pos: Position, msg: String) -> Self {
        Self { pos, msg }
    }
}

impl From<LexicalError> for SyntaxError {
    fn from(value: LexicalError) -> Self {
        Self::new(value.pos, value.msg)
    }
}

impl SyntaxErrorList {
    #[must_use]
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn combine(&mut self, other: SyntaxErrorList) {
        self.list.extend(other.list);
    }
}

impl From<SyntaxError> for SyntaxErrorList {
    fn from(value: SyntaxError) -> Self {
        Self { list: vec![value] }
    }
}

impl From<LexicalError> for SyntaxErrorList {
    fn from(value: LexicalError) -> Self {
        Self::from(SyntaxError::from(value))
    }
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;
pub type SyntaxResultList<T> = Result<T, SyntaxErrorList>;
