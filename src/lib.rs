#![crate_id = "brainfuck"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(macro_rules, phase)]

//! dox (placeholder)

#[phase(syntax, link)] extern crate log;
#[cfg(test)] extern crate test;
extern crate collections;

// Re-export
pub use storage::{Unit, Tape, VectorTape, SparseTape};
pub use operators::Operator;
pub use ast::Ast;
pub use machine::Machine;

pub mod storage;
pub mod operators;
pub mod ast;
pub mod machine;
