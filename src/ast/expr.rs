//! The module of `Expr`

use crate::ast::binary_ope::BinaryOperator;
use crate::ast::unary_ope::UnaryOperator;
use crate::compiler_data::position::Position;
use crate::compiler_data::value::Value;

pub struct Expr {
    pub pos: Position,
    pub expr_type: ExprType,
}

pub enum ExprType {
    Literal(ExprLiteral),
    Unary(ExprUnary),
    Binary(ExprBinary),
}

pub struct ExprLiteral {
    pub value: Value,
}

pub struct ExprUnary {
    pub ope: UnaryOperator,
    pub rhs: Box<Expr>,
}

pub struct ExprBinary {
    pub ope: BinaryOperator,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}
