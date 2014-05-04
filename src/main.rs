#![crate_id = "interpreter"]
#![crate_type = "bin"]
#![feature(macro_rules, phase)]

#[phase(syntax, link)] extern crate log;
extern crate brainfuck;
extern crate getopts;

use std::os;
use std::io::File;
use getopts::getopts;
use brainfuck::{Ast,Machine};

/// Prints a simple help screen.
fn usage(reason: &str) {
    let cmd = os::args()[0];
    println!("Usage: {} <source.bf>", cmd);
    if !reason.is_empty() { println!("{}", reason); }
}

/// Reads the contents of a file into a string.
fn read_file(filename: &~str) -> Result<~str, ~str> {
    let mut file = match File::open( &Path::new(filename.as_bytes()) ) {
        Ok(f) => f,
        _ => return Err(format!("Cannot open file `{}`.", filename)),
    };
    match file.read_to_str() {
        Ok(s) => Ok(s),
        _ => Err(format!("Could not read contents of file `{}`.", filename)),
    }
}

/// Interpretes the given program, piping from STDIN
/// and to STDOUT.
fn main() {
    let args = os::args();
    let matches = match getopts(args.tail(), []) {
        Ok(m) => m,
        Err(err) => return usage(err.to_err_msg()),
    };
    if matches.free.is_empty() {
        return usage("No source file given.");
    };

    for filename in matches.free.iter() {
        // Read the program source.
        let source = match read_file(filename) {
            Ok(source) => source,
            Err(msg) => return usage(msg),
        };

        // Parse the source code into an AST.
        let program = match Ast::parse_str(source) {
            Ok(program) => program,
            Err(msg) => fail!(msg),
        };

        // Create a machine and run the AST.
        let mut machine = Machine::new();
        match machine.run_program(&program) {
            Ok(_) => { /* nop */ },
            Err(msg) => fail!(msg),
        };
    }
}
