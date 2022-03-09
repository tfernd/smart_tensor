use super::*;
use crate::{Data, Stride};

// TODO move this to a separate file
/// Number of elements in a a shape.
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}

macro_rules! impl_attrs {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D: Data, $(const $s: usize),+, const CANNONICAL: bool> $tensor<D, $($s),+, CANNONICAL>
        where
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            pub(crate) const SHAPE: [usize; $dim] = [$($s),+];
            pub(crate) const NUMEL: usize = shape_numel!([$($s)+]);
            pub(crate) const DIM: usize = $dim;

            #[inline]
            pub const fn shape(&self) -> [usize; $dim] {
                Self::SHAPE
            }

            #[inline]
            pub fn stride(&self) -> Stride<$dim> {
                self.stride.clone()
            }

            #[inline]
            pub const fn numel(&self) -> usize {
                Self::NUMEL
            }

            #[inline]
            pub const fn dim(&self) -> usize {
                Self::DIM
            }

            #[inline]
            pub const fn is_cannonical(&self) -> bool {
                CANNONICAL
            }
        }
    };
}

// TODO move this one level up
impl_attrs!(Tensor1, 1, [S0]);
impl_attrs!(Tensor2, 2, [S0 S1]);
impl_attrs!(Tensor3, 3, [S0 S1 S2]);
impl_attrs!(Tensor4, 4, [S0 S1 S2 S3]);
