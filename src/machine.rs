use std::io::stdio::{stdin_raw, stdout_raw};
use storage::{Tape, VectorTape};
use operators::{Sub, Incr, Decr, Prev, Next, Put, Get};
use ast::Ast;


/**
A brainfuck interpreter machine.

Models the internal state of a Brainfuck machine. It is a simple
tape machine with a program counter representing the current
operator being executed in an AST.
*/
pub struct Machine {
	/// A tape to be used as the main storage.
	tape: VectorTape<u8>,
	/// Program counter pointing at the current operator.
	pc: uint,
}

impl Machine {

	// Produce a new pristine machine.
	pub fn new() -> Machine {
		Machine {
			tape: VectorTape::new(),
			pc: 0,
		}
	}

	/**
	Run a program, given in the form of a parsed AST, on this
	machine's tape. Will return the cycles that have been executed.
	*/
	pub fn run_program<'a>(&mut self, program: &Ast) -> Result<uint, ~str> {
		self.pc = 0; // Begin interpreting at the start of the AST.
		let mut cycles: uint = 0; // Keep track of the executed cycles.
		let Ast(ref ops) = *program; // Extract the actual ops from the AST.

		loop {
			match ops.get(self.pc) {
				// Operations on tape. Match tape methods perfectly.
				Some(&Decr) => { self.tape.mutate( |v|{ *v -= 1; } ); }
				Some(&Incr) => { self.tape.mutate( |v|{ *v += 1; } ); }
				Some(&Prev) => { self.tape.wind(-1); }
				Some(&Next) => { self.tape.wind( 1); }
				// Reads a single char from `stdin` and replaces the
				// current cell's contents with it.
				Some(&Get)  => {
					let byte_in = stdin_raw().read_u8().ok()
						.unwrap_or(0); // This machine respects EOF -> 0
					self.tape.mutate( |v|{ *v = byte_in; } );
				}
				// Prints the cell's contents to `stdout` as char.
				Some(&Put)  => {
					let byte_out = self.tape.cell().clone();
					match stdout_raw().write_u8(byte_out) {
						Ok(_) => { /* nop */ },
						_ => return Err(~"Cannot not write to stdout."),
					}
				}
				// Executes a sub-AST. If the current cell's value
				// is zero, the ops in the sub-AST will be executed,
				// else skipping them entirely.
				Some(&Sub(ref ast)) => {
					let pc = self.pc; // Save PC and reset
					while *self.tape.cell() != 0 {
						match self.run_program(ast) {
							Ok(cls) => cycles += cls,
							Err(msg) => return Err(msg),
						}
					}
					self.pc = pc; // Restore PC
				}
				// Unknown. Nop.
				Some(_) => { /* nop */ },
				// End of program. Stop execution.
				_ => break
			}
			// Track this last cycle and advance to the next operator.
			cycles += 1;
			self.pc += 1;
		}

		// Everything went well. Just return the stats back.
		Ok(cycles)
	}
}

