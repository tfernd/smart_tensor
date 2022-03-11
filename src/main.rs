#![allow(dead_code, unused_variables, unused_imports, incomplete_features)]
#![feature(generic_const_exprs)]

#[macro_use]
mod macros;

mod data;
mod stride;
mod tensor;
mod traits;

pub use data::*;
pub use stride::Stride;
pub use tensor::*;
pub use traits::*;

fn main() {
    let x = StackTensor1::<f32, 100>::full(1.0);
}
