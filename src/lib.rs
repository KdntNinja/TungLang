pub mod eval;
pub mod interpreter;
pub mod parser;
pub mod stdlib;
pub mod value;
pub use crate::interpreter::execute_block;
pub use crate::parser::TungParser;
pub use pest::Parser;
pub use stdlib::StdLib;
