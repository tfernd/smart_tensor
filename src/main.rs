#![allow(dead_code, unused_variables, incomplete_features)]
#![feature(generic_const_exprs)]

#[doc(hidden)]
#[macro_use]
mod macros;

mod data;
mod tensor;
mod traits;

#[doc(hidden)]
pub use data::*;
#[doc(hidden)]
pub use tensor::*;
#[doc(hidden)]
pub use traits::*;

// TODO change main to lib after testing
#[doc(hidden)]
fn main() {
    let x = HeapTensor1::<u8, 100>::empty();

    println!("{:?}", x);
}
