//! The module of `Compiler`

mod error;

use crate::ast::binary_ope::BinaryOperator;
use crate::ast::expr::{Expr, ExprBinary, ExprLiteral, ExprType, ExprUnary};
use crate::compiler::error::{CompileError, CompileResultList};
use crate::compiler_data::data_type::{TypeId, TypeInterner};
use crate::compiler_data::value::{Value, ValueFloat, ValueInteger};
use inkwell::types::AnyTypeEnum;
use inkwell::values::{BasicValue, BasicValueEnum};
use maplit::hashmap;
use std::collections::HashMap;
use std::io::Write;
use inkwell::attributes::{Attribute, AttributeLoc};
use crate::args::{ArgContext, OptionArg};

pub struct Compiler<'ctx> {
    llvm_ctx: &'ctx inkwell::context::Context,
    expr: Box<Expr>,
    module: inkwell::module::Module<'ctx>,
    builder: inkwell::builder::Builder<'ctx>,
    type_list: HashMap<TypeId, AnyTypeEnum<'ctx>>,
    type_interner: TypeInterner,
    output_file: String,
}

type ExprResult<'ctx> = (TypeId, BasicValueEnum<'ctx>);

impl<'ctx> Compiler<'ctx> {
    #[must_use]
    pub fn new(expr: Box<Expr>, output_file: String, ctx: &'ctx inkwell::context::Context) -> Self {
        Self {
            llvm_ctx: ctx,
            expr,
            module: ctx.create_module(&output_file),
            builder: ctx.create_builder(),
            type_list: Self::init_type_list(&ctx),
            type_interner: TypeInterner::create(),
            output_file,
        }
    }

    fn init_type_list(ctx: &inkwell::context::Context) -> HashMap<TypeId, AnyTypeEnum> {
        hashmap! {
            TypeId::VOID => ctx.void_type().into(),
            TypeId::INT8 => ctx.i8_type().into(),
            TypeId::UINT8 => ctx.i8_type().into(),
            TypeId::INT16 => ctx.i16_type().into(),
            TypeId::UINT16 => ctx.i16_type().into(),
            TypeId::INT32 => ctx.i32_type().into(),
            TypeId::UINT32 => ctx.i32_type().into(),
            TypeId::INT64 => ctx.i64_type().into(),
            TypeId::UINT64 => ctx.i64_type().into(),
            TypeId::INT128 => ctx.i128_type().into(),
            TypeId::UINT128 => ctx.i128_type().into(),
            TypeId::FLOAT32 => ctx.f32_type().into(),
            TypeId::FLOAT64 => ctx.f64_type().into(),
        }
    }

    pub fn compile(&self, arg_context: &ArgContext) -> CompileResultList<()> {
        let main_fn_type = self.llvm_ctx.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let entry_block = self.llvm_ctx.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry_block);

        for option in &arg_context.option_args {
            if let OptionArg::NoOptimize = option {
                let optnone = self.llvm_ctx.create_enum_attribute(Attribute::get_named_enum_kind_id("optnone"), 0);
                main_fn.add_attribute(AttributeLoc::Function, optnone);
            }
        }

        let (type_id, val) = self.compile_expression(&self.expr)?;

        match type_id {
            TypeId::VOID => unreachable!(),
            TypeId::INT8 | TypeId::UINT8 | TypeId::INT16 | TypeId::UINT16 | TypeId::INT32 | TypeId::UINT32 | TypeId::INT64 | TypeId::UINT64 => {
                let print_fn_type = self.llvm_ctx.void_type().fn_type(&[self.type_list[&type_id].into_int_type().into()], false);
                let print_fn = self.module.add_function(&format!("std_print_{}", self.type_interner.to_data_type[&type_id]), print_fn_type, None);
                self.builder.build_call(print_fn, &[val.into()], "call").unwrap();
            }
            TypeId::FLOAT32 | TypeId::FLOAT64 => {
                let print_fn_type = self.llvm_ctx.void_type().fn_type(&[self.type_list[&type_id].into_float_type().into()], false);
                let print_fn = self.module.add_function(&format!("std_print_{}", self.type_interner.to_data_type[&type_id]), print_fn_type, None);
                self.builder.build_call(print_fn, &[val.into()], "call").unwrap();
            }
            TypeId::INT128 | TypeId::UINT128 => {
                let print_fn_type = self.llvm_ctx.void_type().fn_type(&[self.llvm_ctx.ptr_type(Default::default()).into()], false);
                let print_fn = self.module.add_function(&format!("std_print_{}", self.type_interner.to_data_type[&type_id]), print_fn_type, None);
                let num_ptr = self.builder.build_alloca(self.type_list[&type_id].into_int_type(), "alloca").unwrap();
                num_ptr.as_instruction_value().unwrap().set_alignment(16).unwrap();
                self.builder.build_store(num_ptr, val.into_int_value()).unwrap();
                self.builder.build_call(print_fn, &[num_ptr.into()], "call").unwrap();
            }
            _ => unreachable!(),
        }

        let println_fn_type = self.llvm_ctx.void_type().fn_type(&[], false);
        let println_fn = self.module.add_function("std_println", println_fn_type, None);
        self.builder.build_call(println_fn, &[], "call").unwrap();

        self.builder.build_return(Some(&self.llvm_ctx.i32_type().const_int(0, false))).unwrap();

        let mut output_bytecode = true;
        for option in &arg_context.option_args {
            if let OptionArg::EmitLLVM = option {
                let path = std::path::Path::new(&self.output_file);
                let output = path.with_extension("ll");
                let mut file = std::fs::File::create(output).unwrap_or_else(|err| {
                    eprintln!("Cannot open or create output file: {}", err);
                    std::process::exit(1);
                });
                file.write_all(self.module.print_to_string().to_bytes()).unwrap_or_else(|err| {
                    eprintln!("Cannot write to output file: {}", err);
                    std::process::exit(1);
                });
                output_bytecode = false;
                break;
            }
        }

        if output_bytecode {
            let path = std::path::Path::new(&self.output_file);
            let output = path.with_extension("bc");
            self.module.print_to_file(output).unwrap_or_else(|err| {
                eprintln!("Cannot write to output file: {}", err);
                std::process::exit(1);
            });
        }

        Ok(())
    }

    fn compile_expression(&self, expr: &Expr) -> CompileResultList<ExprResult> {
        match &expr.expr_type {
            ExprType::Literal(literal) => self.compile_expr_literal(literal),
            ExprType::Unary(unary) => self.compile_expr_unary(expr, unary),
            ExprType::Binary(binary) => self.compile_expr_binary(expr, binary),
        }
    }

    fn compile_expr_literal(&self, literal: &ExprLiteral) -> CompileResultList<ExprResult> {
        match &literal.value {
            Value::Integer(integer) => {
                match integer {
                    ValueInteger::Int8(int) => Ok((TypeId::INT8, self.type_list[&TypeId::INT8].into_int_type().const_int(*int as u64, true).into())),
                    ValueInteger::UInt8(int) => Ok((TypeId::UINT8, self.type_list[&TypeId::UINT8].into_int_type().const_int(*int as u64, false).into())),
                    ValueInteger::Int16(int) => Ok((TypeId::INT16, self.type_list[&TypeId::INT16].into_int_type().const_int(*int as u64, true).into())),
                    ValueInteger::UInt16(int) => Ok((TypeId::UINT16, self.type_list[&TypeId::UINT16].into_int_type().const_int(*int as u64, false).into())),
                    ValueInteger::Int32(int) => Ok((TypeId::INT32, self.type_list[&TypeId::INT32].into_int_type().const_int(*int as u64, true).into())),
                    ValueInteger::UInt32(int) => Ok((TypeId::UINT32, self.type_list[&TypeId::UINT32].into_int_type().const_int(*int as u64, false).into())),
                    ValueInteger::Int64(int) => Ok((TypeId::INT8, self.type_list[&TypeId::INT64].into_int_type().const_int(*int as u64, true).into())),
                    ValueInteger::UInt64(int) => Ok((TypeId::UINT8, self.type_list[&TypeId::UINT64].into_int_type().const_int(*int, false).into())),
                    ValueInteger::Int128(int) => {
                        let bytes = int.to_ne_bytes();
                        let u64s = [u64::from_ne_bytes(bytes[..8].try_into().unwrap()), u64::from_ne_bytes(bytes[8..].try_into().unwrap())];
                        Ok((TypeId::INT128, self.type_list[&TypeId::INT128].into_int_type().const_int_arbitrary_precision(&u64s).into()))
                    }
                    ValueInteger::UInt128(int) => {
                        let bytes = int.to_ne_bytes();
                        let u64s = [u64::from_ne_bytes(bytes[..8].try_into().unwrap()), u64::from_ne_bytes(bytes[8..].try_into().unwrap())];
                        Ok((TypeId::UINT128, self.type_list[&TypeId::UINT128].into_int_type().const_int_arbitrary_precision(&u64s).into()))
                    }
                }
            }
            Value::Float(float) => {
                match float {
                    ValueFloat::Float32(float) => Ok((TypeId::FLOAT32, self.type_list[&TypeId::FLOAT32].into_float_type().const_float(*float as f64).into())),
                    ValueFloat::Float64(float) => Ok((TypeId::FLOAT64, self.type_list[&TypeId::FLOAT64].into_float_type().const_float(*float).into())),
                }
            }
        }
    }

    fn compile_expr_unary(&self, expr: &Expr, unary: &ExprUnary) -> CompileResultList<ExprResult> {
        let (rhs_ty, rhs_val) = self.compile_expression(&unary.rhs)?;
        match rhs_ty {
            TypeId::INT8 | TypeId::INT16 | TypeId::INT32 | TypeId::INT64 | TypeId::INT128 => Ok((rhs_ty, self.builder.build_int_neg(rhs_val.into_int_value(), "neg").unwrap().into())),
            TypeId::FLOAT32 | TypeId::FLOAT64 => Ok((rhs_ty, self.builder.build_float_neg(rhs_val.into_float_value(), "neg").unwrap().into())),
            _ => Err(CompileError::new(expr.pos.clone(), format!("Cannot use a negative sign on type '{}'.", self.type_interner.to_data_type[&rhs_ty])).into()),
        }
    }

    fn compile_expr_binary(&self, expr: &Expr, binary: &ExprBinary) -> CompileResultList<ExprResult> {
        let (lhs_ty, lhs_val) = self.compile_expression(&binary.lhs)?;
        let (rhs_ty, rhs_val) = self.compile_expression(&binary.rhs)?;
        if lhs_ty != rhs_ty {
            Err(CompileError::new(expr.pos.clone(), format!("Expected the same types, but found '{}' and '{}'.", self.type_interner.to_data_type[&lhs_ty], self.type_interner.to_data_type[&rhs_ty])).into())
        } else {
            match &binary.ope {
                BinaryOperator::Plus => {
                    match lhs_ty {
                        TypeId::INT8 | TypeId::UINT8 | TypeId::INT16 | TypeId::UINT16 | TypeId::INT32 |
                        TypeId::UINT32 | TypeId::INT64 | TypeId::UINT64 | TypeId::INT128 | TypeId::UINT128 =>
                            Ok((lhs_ty, self.builder.build_int_add(lhs_val.into_int_value(), rhs_val.into_int_value(), "add").unwrap().into())),
                        TypeId::FLOAT32 | TypeId::FLOAT64 => Ok((lhs_ty, self.builder.build_float_add(lhs_val.into_float_value(), rhs_val.into_float_value(), "add").unwrap().into())),
                        _ => unreachable!(),
                    }
                }
                BinaryOperator::Minus => {
                    match lhs_ty {
                        TypeId::INT8 | TypeId::UINT8 | TypeId::INT16 | TypeId::UINT16 | TypeId::INT32 |
                        TypeId::UINT32 | TypeId::INT64 | TypeId::UINT64 | TypeId::INT128 | TypeId::UINT128 =>
                            Ok((lhs_ty, self.builder.build_int_sub(lhs_val.into_int_value(), rhs_val.into_int_value(), "sub").unwrap().into())),
                        TypeId::FLOAT32 | TypeId::FLOAT64 => Ok((lhs_ty, self.builder.build_float_sub(lhs_val.into_float_value(), rhs_val.into_float_value(), "sub").unwrap().into())),
                        _ => unreachable!(),
                    }
                }
                BinaryOperator::Multiply => {
                    match lhs_ty {
                        TypeId::INT8 | TypeId::UINT8 | TypeId::INT16 | TypeId::UINT16 | TypeId::INT32 |
                        TypeId::UINT32 | TypeId::INT64 | TypeId::UINT64 | TypeId::INT128 | TypeId::UINT128 =>
                            Ok((lhs_ty, self.builder.build_int_mul(lhs_val.into_int_value(), rhs_val.into_int_value(), "mul").unwrap().into())),
                        TypeId::FLOAT32 | TypeId::FLOAT64 => Ok((lhs_ty, self.builder.build_float_mul(lhs_val.into_float_value(), rhs_val.into_float_value(), "add").unwrap().into())),
                        _ => unreachable!(),
                    }
                }
                BinaryOperator::Divide => {
                    match lhs_ty {
                        TypeId::INT8 | TypeId::INT16 | TypeId::INT32 | TypeId::INT64 | TypeId::INT128 => Ok((lhs_ty, self.builder.build_int_signed_div(lhs_val.into_int_value(), rhs_val.into_int_value(), "sdiv").unwrap().into())),
                        TypeId::UINT8 | TypeId::UINT16 | TypeId::UINT32 | TypeId::UINT64 | TypeId::UINT128 => Ok((lhs_ty, self.builder.build_int_unsigned_div(lhs_val.into_int_value(), rhs_val.into_int_value(), "udiv").unwrap().into())),
                        TypeId::FLOAT32 | TypeId::FLOAT64 => Ok((lhs_ty, self.builder.build_float_div(lhs_val.into_float_value(), rhs_val.into_float_value(), "div").unwrap().into())),
                        _ => unreachable!(),
                    }
                }
                BinaryOperator::Power => unimplemented!("Waiting for function support."),
            }
        }
    }
}
