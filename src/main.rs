// #![allow(dead_code, unused_imports, unused_variables)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, const_fn_trait_bound)]
#![feature(trace_macros)]

mod stride;
mod tensor;
mod traits;

pub use stride::Stride;
pub use tensor::*;
pub use traits::*;

fn main() {
    let x = TensorStack2::<u8, 10, 10>::empty();
    let y = x.view4::<1, 1, 10, 10>();

    println!("{:?}", y);
    println!("{:?}", y.stride());
}
