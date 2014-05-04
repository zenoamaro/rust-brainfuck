use std::fmt;
use std::from_str::FromStr;
use operators::{Operator, Sub, Skip, Loop};


/**
The internal parsed representation of a program source.
*/
pub struct Ast(~[Operator]);

impl Ast {
	/**
	Produce an AST from a source string.
	This is the most commod method to generate an Ast.
	*/
	pub fn parse_str(source: &str) -> Result<Ast, ~str> {
		/*
		We parse loops by making a context to group its operators,
		pushing on it until the matching loop end. As we create the
		context, we push the previous one onto a stack. After the
		nest has been collected, we pop the context and replace it
		with the subprocess operator.
		*/
		let mut stack:~[ ~[Operator] ] = ~[];
		let mut ops: ~[Operator] = ~[];

		for token in source.chars() {
			match from_str::<Operator>(token.to_str()) {
				/*
				Start of a loop. Produce a new context in which
				to push operators, and push the old one on the
				stack.
				*/
				Some(Skip) => {
					stack.push(ops);
					ops = ~[];
				}
				/*
				End of a loop. Make a subprocess operator out of
				the just-collected context, and push that on the
				previous context.
				*/
				Some(Loop) => {
					let sub_ast = Sub(Ast( ops ));
					// Try to pop the previous context from the stack.
					// If this does not work, it's an unmatched `]`.
					ops = match stack.pop() {
						Some(ops) => ops,
						_ => return Err(~"Unmatched `]`."),
					};
					ops.push(sub_ast);
				}
				// Push the operator onto the context.
				Some(op) => ops.push(op),
				// Unknown. Probably comments. Nop.
				_ => continue
			}
		}

		// If we still have things on the stack, then we have one or
		// more unmatched `[`.
		if ! stack.is_empty() {
			return Err(~"Unmatched `[`.");
		}

		// Everything went well.
		return Ok(Ast(ops));
	}
}

impl FromStr for Ast {
	fn from_str(source: &str) -> Option<Ast> {
		Ast::parse_str(source).ok()
	}
}

impl fmt::Show for Ast {
	/**
	Parses a string into the matching operator.
	*/
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		let &Ast(ref ops) = self;
		let display = |op: &Operator| -> ~str { format!("{}", op) };
		let repr: ~[~str] = ops.iter().map(display).collect();
		f.buf.write(format!("{}", repr.concat()).as_bytes()
		)
	}
}
