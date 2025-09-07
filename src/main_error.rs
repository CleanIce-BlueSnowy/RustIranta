//! The module for handling errors.

use crate::compiler_data::position::Position;

pub trait IrantaCompilerError {
    #[must_use]
    fn get_pos(&self) -> &Position;
    #[must_use]
    fn get_msg(&self) -> &str;
}

pub struct CopySource<'a> {
    pub lines: Vec<&'a str>,
}

pub fn print_error(error_type: &str, error: impl IrantaCompilerError, copy_source: &CopySource) {
    let pos = error.get_pos();
    let msg = error.get_msg();

    println!("{} {} {}", error_type, pos, msg);

    let line = copy_source.lines[pos.start_line - 1];
    println!("> {}", line);
    print!("  ");
    for _ in 1..pos.start_col {
        print!(" ");
    }
    if pos.start_line == pos.end_line {
        for _ in pos.start_col..=pos.end_col {
            print!("^");
        }
        println!();
    } else {
        for _ in pos.start_col..=line.len() {
            print!("^");
        }
    }

    if pos.end_line > pos.start_line {
        if pos.end_line - pos.start_line > 1 {
            println!("> ......");
        }
        let line = copy_source.lines[pos.end_line - 1];
        println!("> {}", line);
        print!("  ");
        for _ in 1..=pos.end_col {
            print!("^");
        }
        println!();
    }
}
