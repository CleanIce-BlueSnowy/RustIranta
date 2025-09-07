//! The module of `Parser`

pub mod error;

use std::collections::HashMap;
use maplit::hashmap;
use crate::args::{ArgContext, OptionArg};
use crate::ast::binary_ope::BinaryOperator;
use crate::ast::expr::{Expr, ExprBinary, ExprLiteral, ExprType, ExprUnary};
use crate::ast::unary_ope::UnaryOperator;
use crate::compiler_data::position::Position;
use crate::compiler_data::value::{Value, ValueFloat, ValueInteger};
use crate::lexer::context::LexerContext;
use crate::lexer::token::{Token, TokenFloat, TokenInteger, TokenLiteral, TokenOperator, TokenParen, TokenType};
use crate::lexer::Lexer;
use crate::parser::error::{SyntaxError, SyntaxResultList};

pub struct Parser<'a> {
    lexer_ctx: LexerContext,
    source: &'a String,
    lexer: Lexer,
    precedence: HashMap<TokenOperator, (u32, u32)>
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(source: &'a String, arg_ctx: &ArgContext) -> Self {
        Self {
            lexer_ctx: LexerContext {
                debug_print_token: {
                    let mut found = false;
                    for option_arg in &arg_ctx.option_args {
                        if let OptionArg::DebugPrintToken = option_arg {
                            found = true;
                            break;
                        }
                    }
                    found
                }
            },
            source,
            lexer: Lexer::new(source),
            precedence: Self::init_precedence(),
        }
    }

    #[must_use]
    fn init_precedence() -> HashMap<TokenOperator, (u32, u32)> {
        hashmap! {
            TokenOperator::Plus => (50, 51),
            TokenOperator::Minus => (50, 51),
            TokenOperator::Multiply => (60, 61),
            TokenOperator::Divide => (60, 61),
            TokenOperator::Power => (71, 70),
        }
    }

    pub fn parse(&mut self) -> SyntaxResultList<Box<Expr>> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, last_rbp: u32) -> SyntaxResultList<Box<Expr>> {
        let token = self.lexer.get_token(&self.lexer_ctx)?;
        let mut left = self.parse_expr_nud(&token)?;

        let mut current_token = self.lexer.peek_token(&self.lexer_ctx)?;
        while !current_token.is_eof() {
            match &current_token.token_type {
                TokenType::Operator(ope) => {
                    let operator = ope.clone();
                    let (lbp, rbp) = self.precedence[ope];
                    if lbp < last_rbp {
                        break;
                    }
                    let pos = current_token.pos.clone();
                    self.lexer.get_token(&self.lexer_ctx)?;
                    left = self.parse_expr_led(&pos, &operator, left, rbp)?;
                }
                _ => return Err(SyntaxError::new(self.lexer.peek_token(&self.lexer_ctx)?.pos.clone(), "Unexpected token in an expression.".to_string()).into()),
            }
            current_token = self.lexer.peek_token(&self.lexer_ctx)?;
        }

        Ok(left)
    }

    fn parse_expr_nud(&mut self, token: &Token) -> SyntaxResultList<Box<Expr>> {
        match &token.token_type {
            TokenType::Literal(literal) => Ok(Box::new(Expr {
                pos: token.pos.clone(),
                expr_type: ExprType::Literal(ExprLiteral {
                    value: Self::token_literal_to_value(&literal),
                })
            })),
            TokenType::Paren(TokenParen::LeftParen) => {
                let expr = self.parse_expression(0)?;
                if !matches!(self.lexer.peek_token(&self.lexer_ctx)?.token_type, TokenType::Paren(TokenParen::RightParen)) {
                    let err_token = self.lexer.get_token(&self.lexer_ctx)?;
                    return Err(SyntaxError::new(err_token.pos.clone(), "Expect ')'.".to_string()).into());
                }
                let paren_token = self.lexer.get_token(&self.lexer_ctx)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&token.pos, &paren_token.pos),
                    expr_type: expr.expr_type,
                }))
            }
            TokenType::Operator(TokenOperator::Plus) => {
                let (lbp, _rbp) = self.precedence[&TokenOperator::Plus];
                let expr = self.parse_expression(lbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&token.pos, &expr.pos),
                    expr_type: ExprType::Unary(ExprUnary {
                        ope: UnaryOperator::Plus,
                        rhs: expr,
                    })
                }))
            }
            TokenType::Operator(TokenOperator::Minus) => {
                let (lbp, _rbp) = self.precedence[&TokenOperator::Minus];
                let expr = self.parse_expression(lbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&token.pos, &expr.pos),
                    expr_type: ExprType::Unary(ExprUnary {
                        ope: UnaryOperator::Minus,
                        rhs: expr,
                    })
                }))
            }
            _ => Err(SyntaxError::new(token.pos.clone(), "Unexpected token in an expression.".to_string()).into()),
        }
    }

    fn parse_expr_led(&mut self, _pos: &Position, operator: &TokenOperator, lhs: Box<Expr>, rbp: u32) -> SyntaxResultList<Box<Expr>> {
        match operator {
            TokenOperator::Plus => {
                let rhs = self.parse_expression(rbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&lhs.pos, &rhs.pos),
                    expr_type: ExprType::Binary(ExprBinary {
                        ope: BinaryOperator::Plus,
                        lhs,
                        rhs,
                    })
                }))
            }
            TokenOperator::Minus => {
                let rhs = self.parse_expression(rbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&lhs.pos, &rhs.pos),
                    expr_type: ExprType::Binary(ExprBinary {
                        ope: BinaryOperator::Minus,
                        lhs,
                        rhs,
                    })
                }))
            }
            TokenOperator::Multiply => {
                let rhs = self.parse_expression(rbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&lhs.pos, &rhs.pos),
                    expr_type: ExprType::Binary(ExprBinary {
                        ope: BinaryOperator::Multiply,
                        lhs,
                        rhs,
                    })
                }))
            }
            TokenOperator::Divide => {
                let rhs = self.parse_expression(rbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&lhs.pos, &rhs.pos),
                    expr_type: ExprType::Binary(ExprBinary {
                        ope: BinaryOperator::Divide,
                        lhs,
                        rhs,
                    })
                }))
            }
            TokenOperator::Power => {
                let rhs = self.parse_expression(rbp)?;
                Ok(Box::new(Expr {
                    pos: Position::combine(&lhs.pos, &rhs.pos),
                    expr_type: ExprType::Binary(ExprBinary {
                        ope: BinaryOperator::Power,
                        lhs,
                        rhs,
                    })
                }))
            }
        }
    }

    fn token_literal_to_value(literal: &TokenLiteral) -> Value {
        match literal {
            TokenLiteral::Integer(integer) => {
                match integer {
                    TokenInteger::Int8(int) => Value::Integer(ValueInteger::Int8(*int)),
                    TokenInteger::UInt8(int) => Value::Integer(ValueInteger::UInt8(*int)),
                    TokenInteger::Int16(int) => Value::Integer(ValueInteger::Int16(*int)),
                    TokenInteger::UInt16(int) => Value::Integer(ValueInteger::UInt16(*int)),
                    TokenInteger::Int32(int) => Value::Integer(ValueInteger::Int32(*int)),
                    TokenInteger::UInt32(int) => Value::Integer(ValueInteger::UInt32(*int)),
                    TokenInteger::Int64(int) => Value::Integer(ValueInteger::Int64(*int)),
                    TokenInteger::UInt64(int) => Value::Integer(ValueInteger::UInt64(*int)),
                    TokenInteger::Int128(int) => Value::Integer(ValueInteger::Int128(*int)),
                    TokenInteger::UInt128(int) => Value::Integer(ValueInteger::UInt128(*int)),
                }
            }
            TokenLiteral::Float(float) => {
                match float {
                    TokenFloat::Float32(float) => Value::Float(ValueFloat::Float32(*float)),
                    TokenFloat::Float64(float) => Value::Float(ValueFloat::Float64(*float)),
                }
            }
        }
    }
}
