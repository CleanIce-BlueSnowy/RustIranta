//! The module of `Token`

//! A token that includes a type and position information

use std::fmt::Display;
use crate::compiler_data::position::Position;

pub struct Token {
    pub pos: Position,
    pub token_type: TokenType,
}

impl Token {
    #[must_use]
    pub fn new(pos: Position, token_type: TokenType) -> Self {
        Self {
            pos,
            token_type,
        }
    }

    #[must_use]
    pub fn is_eof(&self) -> bool {
        matches!(self.token_type, TokenType::EOF)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN {} {:?}", self.pos, self.token_type)
    }
}

#[derive(Debug)]
pub enum TokenType {
    Literal(TokenLiteral),
    Identifier(String),
    Operator(TokenOperator),
    Paren(TokenParen),
    EOF,
}

#[derive(Debug)]
pub enum TokenLiteral {
    Integer(TokenInteger),
    Float(TokenFloat),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TokenOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug)]
pub enum TokenParen {
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum TokenInteger {
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Int128(i128),
    UInt128(u128),
}

#[derive(Debug)]
pub enum TokenFloat {
    Float32(f32),
    Float64(f64),
}
