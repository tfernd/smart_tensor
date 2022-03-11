use num_traits::{One, Zero};

mod aliases;
pub use aliases::*;

use crate::stride::Stride;
use crate::traits::{Data, EmptyData, FullData, FullLikeData};
use crate::traits::{EmptyTensor, FullTensor, Tensor};

macro_rules! make_tensor {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        /// A n-dimensional tensor.
        #[derive(Debug)]
        pub struct $tensor<D: Data, $(const $s: usize),+, const CANNONICAL: bool> {
            pub(crate) data: D,
            pub(crate) stride: Stride<$dim>
        }

        impl<D: Data, $(const $s: usize),+, const CANNONICAL: bool> Tensor<$dim> for $tensor<D, $($s),+, CANNONICAL> {
            type Data = D;

            const SHAPE: [usize; $dim] = [$($s),+];

            #[inline]
            unsafe fn new(data: Self::Data) -> Self {
                let stride = Stride::default_stride(Self::SHAPE);

                Self { data, stride }
            }
        }

        impl<D: EmptyData, $(const $s: usize),+> EmptyTensor<$dim> for $tensor<D, $($s),+, true> {}

        impl<D: FullData, $(const $s: usize),+> FullTensor<$dim> for $tensor<D, $($s),+, true>
        where
            D::Item: Copy + Zero + One, {}
    }
}

make_tensor!(Tensor1, 1, [S0]);
make_tensor!(Tensor2, 2, [S0 S1]);
make_tensor!(Tensor3, 3, [S0 S1 S2]);
make_tensor!(Tensor4, 4, [S0 S1 S2 S3]);
