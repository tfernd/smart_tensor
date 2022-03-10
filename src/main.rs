#![allow(dead_code, incomplete_features)]
#![feature(generic_const_exprs)]

#[macro_use]
mod macros;

mod data;
mod stride;
mod tensor;
mod traits;
pub use data::*;
pub use stride::*;
pub use tensor::*;
pub use traits::*;

fn main() {}
