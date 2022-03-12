#![allow(dead_code, unused_variables, unused_imports, incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(trace_macros)]

#[macro_use]
mod macros;

mod data;
mod stride;
mod traits;
// mod tensor;

pub use data::*;
pub use stride::Stride;
pub use traits::*;
// pub use tensor::*;

use std::ops::Add;

// ! make macro for different ops
// ? Stack + Stack and Heap + heap, but is Stack + Heap = Heap?
impl<T, const NUMEL: usize> Add<&StackData<T, NUMEL>> for &StackData<T, NUMEL> {
    type Output = StackData<T, NUMEL>;

    fn add(self, rhs: &StackData<T, NUMEL>) -> Self::Output {
        let out = Self::Output::empty();

        for i in 0..NUMEL {
            // TODO implement get_uncheked and set_unchecked
            // out.0[i] = self.0[i] + rhs.0[i];
        }

        out
    }
}

fn main() {
    let x = StackData::<f32, 10>::ones();
    let y = StackData::<f32, 10>::zeros();
}
