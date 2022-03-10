macro_rules! def_attrs {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D: Data, $(const $s: usize),+, const CANNONICAL: bool> TensorAttributes<$dim> for $tensor<D, $($s),+, CANNONICAL>
        {
            const SHAPE: [usize; $dim] = [$($s),+];
            const NUMEL: usize = shape_numel!([$($s)+]);

            #[inline]
            fn stride(&self) -> Stride<$dim> {
                self.stride.clone()
            }

            #[inline]
            fn is_cannonical(&self) -> bool {
                CANNONICAL
            }
        }
    };
}
