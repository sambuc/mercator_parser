#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub queries); // synthesized by LALRPOP

pub mod ast;
pub use ast::*;

#[cfg(test)]
mod tests;
