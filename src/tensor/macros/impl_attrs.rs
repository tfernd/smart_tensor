macro_rules! impl_attrs {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D, $(const $s: usize),+, const CANNONICAL: bool> TensorAttributes<$dim> for $tensor<D, $($s),+, CANNONICAL>
        where
            D: Data,
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            const SHAPE: [usize; $dim] = [$($s),+];
            const NUMEL: usize = shape_numel!([$($s)+]);
            const DIM: usize = $dim;

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
