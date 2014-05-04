use std::fmt;
use std::from_str::FromStr;
use ast::Ast;


/**
Internal representations for actual language operators.
*/

pub enum Operator {

	// Operators from the spec:

	/// Increments the contents of the cell by 1.
	Incr,
	/// Decrements the contents of the cell by 1.
	Decr,
	/// Moves the tape head one cell to the left.
	Prev,
	/// Moves the tape head one cell to the right.
	Next,
	/// Print the contents of the cell to `stdout` as a char.
	Put,
	/// Inputs the contents of the cell from `stdin` as a char.
	Get,

	/// If the cell under head is zero, jump to matching `Loop`.
	Skip,
	/// If the cell under head is non-zero, jump to matching `Skip`.
	Loop,
	/// An ignored, extraneous character.
	Nop(~str),

	// Internal operators:

	/// Contains the parsed Ast for a nested block of code.
	/// Used for containing the code inside `[...]` loops.
	Sub(Ast),

}

impl FromStr for Operator {
	/**
	Converts an operator to its string representation.
	*/
	fn from_str(op: &str) -> Option<Operator> {
		match op {
			"+" => Some(Incr),
			"-" => Some(Decr),
			"<" => Some(Prev),
			">" => Some(Next),
			"[" => Some(Skip),
			"]" => Some(Loop),
			"." => Some(Put),
			"," => Some(Get),
			c => Some(Nop(c.to_owned())),
		}
	}
}

impl fmt::Show for Operator {
	/**
	Parses a string into the matching operator.
	*/
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		f.buf.write((match *self {
			Incr => ~"+",
			Decr => ~"-",
			Prev => ~"<",
			Next => ~">",
			Skip => ~"[",
			Loop => ~"]",
			Put  => ~".",
			Get  => ~",",
			Nop(ref c) => c.to_owned(),
			Sub(ref ast) => format!("[{}]", ast),
		}).as_bytes())
	}
}
