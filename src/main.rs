#![allow(dead_code, unused_variables, incomplete_features)]
#![feature(generic_const_exprs)]

#[macro_use]
mod macros;

mod data;
mod tensor;
mod traits;

pub use data::*;
pub use tensor::*;
pub use traits::*;

fn main() {
    let x = HeapTensor1::<u8, 100>::empty();

    println!("{:?}", x);
}
