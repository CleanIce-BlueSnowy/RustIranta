//! The module of `AstPrinter`

use crate::ast::expr::{Expr, ExprBinary, ExprLiteral, ExprType, ExprUnary};
use crate::compiler_data::position::Position;

pub struct AstPrinter {}

impl AstPrinter {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn print_expr(&self, expr: &Expr) -> String {
        self.indent(
            &match &expr.expr_type {
                ExprType::Literal(literal) => self.print_expr_literal(&expr.pos, &literal),
                ExprType::Unary(unary) => self.print_expr_unary(&expr.pos, &unary),
                ExprType::Binary(binary) => self.print_expr_binary(&expr.pos, &binary),
            }
        )
    }

    #[must_use]
    fn print_expr_literal(&self, pos: &Position, literal: &ExprLiteral) -> String {
        format!(
            "{} Expr::Literal {{ value: {} }} ",
            pos,
            literal.value
        )
    }

    #[must_use]
    fn print_expr_unary(&self, pos: &Position, unary: &ExprUnary) -> String {
        format!(
            "{} Expr::Unary {{\n\
                ope: {}\n\
                rhs: {}\n\
            }}",
            pos,
            unary.ope,
            self.print_expr(&unary.rhs),
        )
    }

    #[must_use]
    fn print_expr_binary(&self, pos: &Position, binary: &ExprBinary) -> String {
        format!(
            "{} Expr::Binary {{\n\
                ope: {}\n\
                lhs: {}\n\
                rhs: {}\n\
            }}",
            pos,
            binary.ope,
            self.print_expr(&binary.lhs),
            self.print_expr(&binary.rhs),
        )
    }

    #[must_use]
    fn indent(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len() + text.matches('\n').count() * 4);
        let line_cnt = text.lines().count();
        for (i, line) in text.lines().enumerate() {
            match i {
                0 => out.push_str(&format!("{}\n", line)),
                num if num == line_cnt - 1 => out.push_str(line),
                _ => out.push_str(&format!("    {}\n", line)),
            }
        }
        out.trim().to_string()
    }
}
