use num_traits::{One, Zero};

mod aliases;
#[doc(hidden)]
pub use aliases::*;

use crate::traits::{
    DataEmptyTrait, DataFullTrait, DataTrait, TensorEmptyTrait, TensorFullTrait, TensorTrait,
};

#[doc(hidden)]
macro_rules! make_tensor {
    ( $tensor:ident, [$($s:tt)+] ) => {
        /// A n-dimensional tensor.
        #[derive(Debug)]
        pub struct $tensor<D: DataTrait, $(const $s: usize),+> {
            pub(crate) data: D,
        }

        impl<D: DataTrait, $(const $s: usize),+> TensorTrait for $tensor<D, $($s),+> {
            type Data = D;

            #[inline]
            unsafe fn new(data: Self::Data) -> Self {
                Self { data }
            }
        }

        impl<D: DataEmptyTrait, $(const $s: usize),+> TensorEmptyTrait for $tensor<D, $($s),+> {}

        impl<D: DataFullTrait, $(const $s: usize),+> TensorFullTrait for $tensor<D, $($s),+>
        where
            D::Type: Copy + Zero + One, {}

    }
}

make_tensor!(Tensor1, [S0]);
make_tensor!(Tensor2, [S0 S1]);
make_tensor!(Tensor3, [S0 S1 S2]);
make_tensor!(Tensor4, [S0 S1 S2 S3]);
make_tensor!(Tensor5, [S0 S1 S2 S3 S4]);
