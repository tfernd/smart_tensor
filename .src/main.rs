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

use std::ops::Add;

impl<T, D, const NUMEL: usize> Add<D> for StackData<T, NUMEL>
where
    D: Data<Item = T>,
    T: Add<T>,
{
    type Output = StackData<T, NUMEL>;

    fn add(self, rhs: D) -> Self::Output {
        let a = self.data();
        let b = rhs.data();

        todo!()

        // let out = self
        //     .data()
        //     .iter()
        //     .zip(rhs.data().iter())
        //     .map(|(&x, &y)| x + y)
        //     .collect();

        // unsafe { StackData(out) }
    }
}

fn main() {
    let mut x = HeapData::<u8, 10>::empty();

    for v in x.0.iter_mut() {
        *v = 1;
    }

    println!("{:?}", x);
}
