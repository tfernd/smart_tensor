macro_rules! make_tensor {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
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
    }
}
