use super::*;
use crate::Tensor;

macro_rules! impl_tensor {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D, $(const $s: usize),+> Tensor<$dim> for $tensor<D, $($s),+>
        where
            D: Data,
            [(); D::NUMEL - shape_numel!([$($s)+])]: Sized,
            [(); shape_numel!([$($s)+]) - D::NUMEL]: Sized
        {
            type Type = D::Type;

            const NUMEL: usize = D::NUMEL;
            const SHAPE: [usize; $dim] = [$($s),+];
        }
    };
}

impl_tensor!(Tensor1, 1, [S0]);
impl_tensor!(Tensor2, 2, [S0 S1]);
impl_tensor!(Tensor3, 3, [S0 S1 S2]);
impl_tensor!(Tensor4, 4, [S0 S1 S2 S3]);
