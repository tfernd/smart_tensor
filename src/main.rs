#![allow(dead_code, unused_imports, unused_variables)]
#![allow(incomplete_features, const_evaluatable_unchecked)]
#![feature(generic_const_exprs, const_fn_trait_bound, const_trait_impl)]
#![feature(trace_macros)]

mod stride;
mod tensor;
mod traits;

pub use stride::Stride;
pub use traits::{Data, TensorAttributes};

fn main() {}
