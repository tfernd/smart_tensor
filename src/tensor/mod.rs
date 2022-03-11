use num_traits::{One, Zero};

mod aliases;
#[doc(hidden)]
pub use aliases::*;

use crate::traits::{
    DataEmptyTrait, DataFullTrait, DataTrait, TensorEmptyTrait, TensorFullTrait, TensorTrait,
};
use crate::Stride;

#[doc(hidden)]
macro_rules! make_tensor {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        /// A n-dimensional tensor.
        #[derive(Debug)]
        pub struct $tensor<D: DataTrait, $(const $s: usize),+> {
            pub(crate) data: D,
            pub(crate) stride: Stride<$dim>
        }

        impl<D: DataTrait, $(const $s: usize),+> TensorTrait<$dim> for $tensor<D, $($s),+> {
            type Data = D;

            const SHAPE: [usize; $dim] = [$($s),+];

            #[inline]
            unsafe fn new(data: Self::Data) -> Self {
                let stride = Stride::default_stride(Self::SHAPE);

                Self { data, stride }
            }
        }

        impl<D: DataEmptyTrait, $(const $s: usize),+> TensorEmptyTrait<$dim> for $tensor<D, $($s),+> {}

        impl<D: DataFullTrait, $(const $s: usize),+> TensorFullTrait<$dim> for $tensor<D, $($s),+>
        where
            D::Type: Copy + Zero + One, {}

    }
}

make_tensor!(Tensor1, 1, [S0]);
make_tensor!(Tensor2, 2, [S0 S1]);
make_tensor!(Tensor3, 3, [S0 S1 S2]);
make_tensor!(Tensor4, 4, [S0 S1 S2 S3]);
make_tensor!(Tensor5, 5, [S0 S1 S2 S3 S4]);
