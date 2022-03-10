macro_rules! impl_new {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D, $(const $s: usize),+, const CANNONICAL: bool> $tensor<D, $($s),+, CANNONICAL>
        where
            D: Data,
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            pub fn new(data: D) -> Self {
                assert!(data.len() == Self::NUMEL);

                let stride = Stride::default_stride(Self::SHAPE);
                Self { data, stride }
            }
        }
    };
}
