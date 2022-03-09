use super::*;
use crate::{Data, Stride};

macro_rules! impl_new {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D: Data, $(const $s: usize),+, const CANNONICAL: bool> $tensor<D, $($s),+, CANNONICAL>
        where
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            pub fn new(data: D) -> Self {
                // TODO: check data size
                // TODO: for [T; N] check N == numel
                // assert!(data.len() == Self::NUMEL);

                let stride = Stride::default_stride(Self::SHAPE);
                Self { data, stride }
            }
        }
    };
}

// TODO move this one level up
impl_new!(Tensor1, 1, [S0]);
impl_new!(Tensor2, 2, [S0 S1]);
impl_new!(Tensor3, 3, [S0 S1 S2]);
impl_new!(Tensor4, 4, [S0 S1 S2 S3]);
