use super::*;
use crate::{Data, Stride};

// TODO move this to a separate file
/// Number of elements in a a shape.
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}

/// Create views from a tensor.
macro_rules! impl_flatten {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D, $(const $s: usize),+> $tensor<D, $($s),+, true>
        where
            D: Data,
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            // TODO implement flatten
            // ! maybe copy parts from view...
            // #[inline]
            // pub fn flatten(&self) -> Tensor1<&[T], shape_numel!([$($s)+]), true>
            // {
            //     todo!()
            //     // $tensor_view::new(self.data $(.$fun())?)
            // }
        }
    };
}

// TODO move this one level up
impl_flatten!(Tensor1, 1, [S0]);
impl_flatten!(Tensor2, 2, [S0 S1]);
impl_flatten!(Tensor3, 3, [S0 S1 S2]);
impl_flatten!(Tensor4, 4, [S0 S1 S2 S3]);
