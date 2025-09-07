//! The module of `CompileError`

use crate::compiler_data::position::Position;
use crate::main_error::IrantaCompilerError;

pub struct CompileError {
    pub pos: Position,
    pub msg: String,
}

pub struct CompileErrorList {
    pub list: Vec<CompileError>,
}

impl IrantaCompilerError for CompileError {
    fn get_pos(&self) -> &Position {
        &self.pos
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }
}

impl CompileError {
    #[must_use]
    pub fn new(pos: Position, msg: String) -> Self {
        Self { pos, msg }
    }
}

impl Into<CompileErrorList> for CompileError {
    fn into(self) -> CompileErrorList {
        CompileErrorList::new(vec![self])
    }
}

impl CompileErrorList {
    #[must_use]
    pub fn new(list: Vec<CompileError>) -> Self {
        Self { list }
    }

    pub fn combine(&mut self, other: CompileErrorList) {
        self.list.extend(other.list);
    }
}

pub type CompileResult<T> = Result<T, CompileError>;
pub type CompileResultList<T> = Result<T, CompileErrorList>;
