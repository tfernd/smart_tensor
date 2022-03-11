#![allow(dead_code, unused_variables, unused_imports, incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(trace_macros)]

mod data;
mod traits;
// mod stride;
// mod tensor;

pub use data::*;
pub use traits::*;
// pub use stride::Stride;
// pub use tensor::*;

// use std::ops::Add;

// impl<T, D, const NUMEL: usize> Add<D> for StackData<T, NUMEL> {
//     type Output = Self;

//     fn add(self, rhs: D) -> Self::Output {
//         todo!()
//     }
// }

fn main() {}
