//! The module of `LexicalError`

use crate::compiler_data::position::Position;
use crate::main_error::IrantaCompilerError;

pub struct LexicalError {
    pub pos: Position,
    pub msg: String,
}

impl IrantaCompilerError for LexicalError {
    fn get_pos(&self) -> &Position {
        &self.pos
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }
}

impl LexicalError {
    #[must_use]
    pub fn new(pos: Position, msg: String) -> Self {
        Self { pos, msg }
    }
}

pub type LexicalResult<T> = Result<T, LexicalError>;
