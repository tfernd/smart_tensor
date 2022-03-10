mod def_aliases;
mod impl_tensor;

use crate::*;
pub use def_aliases::*;

macro_rules! make_tensor {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        pub struct $tensor<D, $(const $s: usize),+>
        {
            data: D,
            stride: Stride<$dim>,
        }
    };
}

make_tensor!(Tensor1, 1, [S0]);
make_tensor!(Tensor2, 2, [S0 S1]);
make_tensor!(Tensor3, 3, [S0 S1 S2]);
make_tensor!(Tensor4, 4, [S0 S1 S2 S3]);
