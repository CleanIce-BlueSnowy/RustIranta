//! The module of `Lexer`

use std::str::FromStr;
use crate::compiler_data::position::Position;
use crate::lexer::context::LexerContext;
use crate::lexer::error::{LexicalError, LexicalResult};
use crate::lexer::token::{Token, TokenFloat, TokenInteger, TokenLiteral, TokenOperator, TokenParen, TokenType};

pub mod token;
pub mod context;
pub mod error;

pub struct Lexer {
    chars: Vec<char>,
    line: usize,
    start_col: usize,
    current_col: usize,
    current: usize,
    buf_token: Option<Token>,
}

impl Lexer {
    #[must_use]
    pub fn new(source: &String) -> Self {
        let chars: Vec<char> = source.chars().collect();

        Self {
            chars,
            line: 1,
            start_col: 1,
            current_col: 1,
            current: 0,
            buf_token: None,
        }
    }

    pub fn peek_token(&mut self, ctx: &LexerContext) -> LexicalResult<&Token> {
        if self.buf_token.is_none() {
            let token = self.scan_token(ctx).or_else(|err| {
                self.synchronize();
                Err(err)
            })?;
            self.buf_token = Some(token);
        }

        Ok(self.buf_token.as_ref().unwrap())
    }

    pub fn get_token(&mut self, ctx: &LexerContext) -> LexicalResult<Token> {
        if self.buf_token.is_none() {
            let token = self.scan_token(ctx).or_else(|err| {
                self.synchronize();
                Err(err)
            })?;
            self.buf_token = Some(token);
        }

        let token = self.buf_token.take().unwrap();
        if ctx.debug_print_token {
            println!("{}", token);
        }
        Ok(token)
    }

    fn scan_token(&mut self, ctx: &LexerContext) -> LexicalResult<Token> {
        self.skip_whitespace();
        self.set_col();

        match self.advance() {
            '\0' => Ok(self.make_token(TokenType::EOF)),
            '+' => Ok(self.make_token(TokenType::Operator(TokenOperator::Plus))),
            '-' => Ok(self.make_token(TokenType::Operator(TokenOperator::Minus))),
            '*' => {
                let ope = if self.match_advance('*') {
                    TokenOperator::Power
                } else {
                    TokenOperator::Multiply
                };
                Ok(self.make_token(TokenType::Operator(ope)))
            }
            '/' => Ok(self.make_token(TokenType::Operator(TokenOperator::Divide))),
            '(' => Ok(self.make_token(TokenType::Paren(TokenParen::LeftParen))),
            ')' => Ok(self.make_token(TokenType::Paren(TokenParen::RightParen))),
            '.' => self.scan_number(ctx),
            ch if ch.is_digit(10) => self.scan_number(ctx),
            _ => Err(LexicalError::new(self.get_current_pos(), "Invalid character.".to_string())),
        }
    }

    fn scan_number(&mut self, _ctx: &LexerContext) -> LexicalResult<Token> {
        let ch = self.last();

        let mut found_dot = false;
        let mut number_str = String::from(ch);
        let mut tag_str = String::new();
        let radix = if ch == '0' && self.peek_next().is_alphabetic() {
            let next = self.peek_next();
            match next {
                'b' => {
                    self.advance();
                    2
                },
                'o' => {
                    self.advance();
                    8
                },
                'x' => {
                    self.advance();
                    16
                },
                _ => 10,
            }
        } else {
            10
        };

        if ch == '.' {
            found_dot = true;
        }

        loop {
            let ch = self.peek_next();
            match ch {
                '.' => {
                    if !found_dot {
                        found_dot = true;
                        number_str.push(ch);
                    } else {
                        return Err(LexicalError::new(self.get_current_pos(), "Unexpected dot.".to_string()))
                    }
                }
                '_' => (),
                ch if ch.is_digit(10) => number_str.push(ch),
                _ => break,
            }
            self.advance();
        }

        loop {
            let ch = self.peek_next();
            match ch {
                '_' => (),
                '.' => return Err(LexicalError::new(self.get_current_pos(), "Unexpected dot.".to_string())),
                ch if ch.is_alphanumeric() => tag_str.push(ch),
                _ => break,
            }
            self.advance();
        }

        if found_dot {
            if number_str == "." {
                return Err(LexicalError::new(self.get_current_pos(), "Invalid syntax.".to_string()));
            } else if number_str.starts_with('.') {
                number_str.insert(0, '0');
            } else if number_str.ends_with('.') {
                number_str.push('0');
            }
        }

        let token_type = match tag_str.as_str() {
            "" => {
                if found_dot {
                    if radix != 10 {
                        return Err(LexicalError::new(self.get_pos(), "Cannot use radix prefix in float type.".to_string()));
                    }
                    let number = f64::from_str(&number_str).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid float64 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Float(TokenFloat::Float64(number)))
                } else {
                    let number = i32::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid int32 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::Int32(number)))
                }
            }
            "int8" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid int8 number: Unexpected dot.".to_string()))
                } else {
                    let number = i8::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid int8 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::Int8(number)))
                }
            }
            "uint8" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid uint8 number: Unexpected dot.".to_string()))
                } else {
                    let number = u8::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid uint8 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::UInt8(number)))
                }
            }
            "int16" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid int16 number: Unexpected dot.".to_string()))
                } else {
                    let number = i16::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid int16 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::Int16(number)))
                }
            }
            "uint16" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid uint16 number: Unexpected dot.".to_string()))
                } else {
                    let number = u16::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid uint16 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::UInt16(number)))
                }
            }
            "int32" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid int32 number: Unexpected dot.".to_string()))
                } else {
                    let number = i32::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid int32 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::Int32(number)))
                }
            }
            "uint32" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid uint32 number: Unexpected dot.".to_string()))
                } else {
                    let number = u32::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid uint32 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::UInt32(number)))
                }
            }
            "int64" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid int64 number: Unexpected dot.".to_string()))
                } else {
                    let number = i64::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid int64 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::Int64(number)))
                }
            }
            "uint64" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid uint64 number: Unexpected dot.".to_string()))
                } else {
                    let number = u64::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid uint64 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::UInt64(number)))
                }
            }
            "int128" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid int128 number: Unexpected dot found.".to_string()))
                } else {
                    let number = i128::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid int128 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::Int128(number)))
                }
            }
            "uint128" => {
                if found_dot {
                    return Err(LexicalError::new(self.get_pos(), "Invalid int8 number: Unexpected dot found.".to_string()))
                } else {
                    let number = u128::from_str_radix(&number_str, radix).or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid uint128 number: {}", err))))?;
                    TokenType::Literal(TokenLiteral::Integer(TokenInteger::UInt128(number)))
                }
            }
            "float32" => {
                if radix != 10 {
                    return Err(LexicalError::new(self.get_pos(), "Cannot use radix prefix in float type.".to_string()));
                }
                let number: f32 = number_str.parse().or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid float32 number: {}", err))))?;
                TokenType::Literal(TokenLiteral::Float(TokenFloat::Float32(number)))
            }
            "float64" => {
                if radix != 10 {
                    return Err(LexicalError::new(self.get_pos(), "Cannot use radix prefix in float type.".to_string()));
                }
                let number: f64 = number_str.parse().or_else(|err| Err(LexicalError::new(self.get_pos(), format!("Invalid float32 number: {}", err))))?;
                TokenType::Literal(TokenLiteral::Float(TokenFloat::Float64(number)))
            }
            _ => return Err(LexicalError::new(self.get_pos(), format!("Invalid number tag: {}", tag_str))),
        };

        Ok(self.make_token(token_type))
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(self.get_pos(), token_type)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.peek_next();
            if ch == '\0' || !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    #[must_use]
    fn peek_next(&self) -> char {
        if self.current < self.chars.len() {
            self.chars[self.current]
        } else {
            '\0'
        }
    }

    #[must_use]
    fn last(&self) -> char {
        if self.current != 0 {
            self.chars[self.current - 1]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) -> char {
        if self.current < self.chars.len() {
            let ch = self.chars[self.current];
            self.current += 1;
            self.current_col += 1;
            if ch == '\n' {
                self.line += 1;
                self.current_col = 1;
            }
            ch
        } else {
            '\0'
        }
    }

    #[must_use]
    fn match_advance(&mut self, ch: char) -> bool {
        if self.peek_next() == ch {
            self.advance();
            true
        } else {
            false
        }
    }

    fn get_pos(&self) -> Position {
        if self.current == self.chars.len() {
            Position {
                start_line: self.line,
                start_col: self.current_col + 1,
                end_line: self.line,
                end_col: self.current_col + 1,
            }
        } else {
            Position {
                start_line: self.line,
                start_col: self.start_col,
                end_line: self.line,
                end_col: self.current_col - 1,
            }
        }
    }

    fn get_current_pos(&self) -> Position {
        if self.current == self.chars.len() {
            Position {
                start_line: self.line,
                start_col: self.current_col + 1,
                end_line: self.line,
                end_col: self.current_col + 1,
            }
        } else {
            Position {
                start_line: self.line,
                start_col: self.current_col - 1,
                end_line: self.line,
                end_col: self.current_col - 1,
            }
        }
    }

    fn set_col(&mut self) {
        self.start_col = self.current_col;
    }

    fn synchronize(&mut self) {
        loop {
            let ch = self.peek_next();
            if ch == '\0' || ch.is_whitespace() {
                break;
            }
            self.advance();
        }
        self.set_col();
    }
}
