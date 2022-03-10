#![allow(dead_code, incomplete_features)]
#![feature(generic_const_exprs)]

#[macro_use]
mod macros;

mod stride;
mod tensor;
mod traits;

pub use stride::Stride;
pub use traits::*;

// pub trait DataGetter: Data {
//     fn get(&self, index: usize) -> Self::Type;
// }

// pub trait DataMutGetter: Data + DataGetter {
//     fn get_mut(&mut self, index: usize) -> Self::Type;
// }

// pub trait DataSetter: Data {
//     fn set(&mut self, index: usize, value: Self::Type);
// }

fn main() {}
