#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod data;
mod traits;

pub use data::*;
pub use traits::*;

fn main() {
    let x = StackData::<u8, 100>::empty();
    let y = &x;

    let y = y.empty_like();

    println!("{:?}", x);
}
