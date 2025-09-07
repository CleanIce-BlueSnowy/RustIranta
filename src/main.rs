//! The main program of Iranta

#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::args::{ArgContext, OptionArg};
use crate::debug::ast_printer::AstPrinter;
use crate::main_error::{print_error, CopySource};
use crate::parser::Parser;
use std::io::Read;
use crate::compiler::Compiler;

mod lexer;
mod compiler_data;
mod main_error;
mod ast;
mod debug;
mod args;
mod parser;
mod compiler;

fn main() {
    println!("Welcome to use Iranta!");

    #[cfg(debug_assertions)]
    {
        println!("[| Debug Version |]")
    }

    let mut iter = std::env::args();
    iter.next();  // Throw the first element.
    let args: Vec<String> = iter.collect();

    let arg_context = ArgContext::parse_args(&args).unwrap_or_else(|err| { eprintln!("Error: {}", err); std::process::exit(1); });

    for option in &arg_context.option_args {
        match option {
            OptionArg::Version => {
                println!(
                    "Iranta 1.0.0-alpha on {} {} [Kernel: {} | Host: {}]",
                    sysinfo::System::name().unwrap_or("UNKNOWN".to_string()),
                    sysinfo::System::os_version().unwrap_or("UNKNOWN".to_string()),
                    sysinfo::System::kernel_version().unwrap_or("UNKNOWN".to_string()),
                    sysinfo::System::host_name().unwrap_or("UNKNOWN".to_string()),
                );
            }
            _ => (),
        }
    }

    if let Some(file_name) = &arg_context.file_name {
        if let Err(()) = compile(file_name, &arg_context) {
            std::process::exit(1);
        }
    }
}

fn compile(source_path: &str, arg_context: &ArgContext) -> Result<(), ()> {
    println!("Compiling...");

    let mut file = std::fs::File::open(source_path).or_else(|err| {
        eprintln!("Cannot open file \"{}\": {}", source_path, err);
        Err(())
    })?;

    let mut source = String::new();
    file.read_to_string(&mut source).or_else(|err| {
        eprintln!("Cannot read file \"{}\": {}", source_path, err);
        Err(())
    })?;

    let copy_source = CopySource {
        lines: source.split("\n").collect(),
    };

    let mut parser = Parser::new(&source, arg_context);
    let expr = parser.parse().or_else(|error| {
        let cnt = error.list.len();
        for err in error.list.into_iter() {
            print_error("Syntax Error", err, &copy_source);
        }
        println!("{} errors in total.", cnt);
        Err(())
    })?;

    for option in &arg_context.option_args {
        if let OptionArg::DebugPrintAST = option {
            let ast_printer = AstPrinter::new();
            println!("{}", ast_printer.print_expr(expr.as_ref()));
            break;
        }
    }

    let mut output_file = std::path::Path::new(source_path);
    for option in &arg_context.option_args {
        if let OptionArg::Output(name) = option {
            output_file = std::path::Path::new(name);
        }
    }

    let llvm_context = inkwell::context::Context::create();
    let compiler = Compiler::new(expr, output_file.to_str().unwrap_or("IRANTA_DEFAULT").to_string(), &llvm_context);
    compiler.compile(&arg_context).or_else(|error| {
        let cnt = error.list.len();
        for err in error.list.into_iter() {
            print_error("Compile Error", err, &copy_source);
        }
        println!("{} errors in total.", cnt);
        Err(())
    })?;

    println!("Finished Compiling Successfully!");

    Ok(())
}
