//! The module for handling arguments from users.

pub struct ArgContext {
    pub option_args: Vec<OptionArg>,
    pub file_name: Option<String>,
}

pub enum OptionArg {
    Version,
    DebugPrintToken,
    DebugPrintAST,
    EmitLLVM,
    Output(String),
    NoOptimize,
}

impl ArgContext {
    pub fn parse_args(args: &[String]) -> Result<Self, String> {
        let mut file_name = None;
        let mut option_args = vec![];
        let mut need_file = true;

        for mut i in 0..args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "-v" | "--version" => {
                    option_args.push(OptionArg::Version);
                    need_file = false;
                }
                "--debug-print-token" => option_args.push(OptionArg::DebugPrintToken),
                "--debug-print-ast" => option_args.push(OptionArg::DebugPrintAST),
                "-o" | "--output" => {
                    if i + 1 == args.len() {
                        return Err("No output file specified".to_string());
                    } else {
                        i += 1;
                        let output = &args[i];
                        option_args.push(OptionArg::Output(output.clone()));
                    }
                }
                "--emit-llvm" => option_args.push(OptionArg::EmitLLVM),
                "-O0" | "--no-optimize" => option_args.push(OptionArg::NoOptimize),
                _ if arg.starts_with("-") => return Err(format!("Invalid argument: {}", arg)),
                _ => file_name = Some(arg.clone()),
            }
        }

        if need_file && file_name.is_none() {
            Err("No source file provided.".to_string())
        } else {
            Ok(Self {
                option_args,
                file_name,
            })
        }
    }
}
