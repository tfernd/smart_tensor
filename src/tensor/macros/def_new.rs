macro_rules! def_new {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl<D: Data, $(const $s: usize),+, const CANNONICAL: bool> $tensor<D, $($s),+, CANNONICAL>
        where
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            pub(crate) fn new(data: D) -> Self {
                // ? Can we remove this and make sure that the data is the correct size?
                assert!(data.len() == Self::NUMEL);

                let stride = Stride::default_stride(Self::SHAPE);
                Self { data, stride }
            }
        }
    }
}
