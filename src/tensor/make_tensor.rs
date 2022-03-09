use crate::{Data, Stride};

macro_rules! make_tensor {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        // TODO replace n by $dim
        /// An n-dimensional tensor.
        #[derive(Debug)]
        pub struct $tensor<D, $(const $s: usize),+, const CANNONICAL: bool>
        where
            D: Data,
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            pub(crate) data: D,
            pub(crate) stride: Stride<$dim>,
        }
    };
}

// TODO move this one level up
make_tensor!(Tensor1, 1, [S0]);
make_tensor!(Tensor2, 2, [S0 S1]);
make_tensor!(Tensor3, 3, [S0 S1 S2]);
make_tensor!(Tensor4, 4, [S0 S1 S2 S3]);
