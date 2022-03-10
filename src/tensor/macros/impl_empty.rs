macro_rules! impl_empty {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        // Owned heap data.
        impl<T, $(const $s: usize),+> TensorEmpty for $tensor<Vec<T>, $($s),+, true>
        where
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            fn empty() -> Self {
                let mut data = Vec::with_capacity(Self::NUMEL);
                unsafe { data.set_len(Self::NUMEL) };

                Self::new(data)
            }
        }

        // Owned stack data.
        impl<T, const SIZE: usize, $(const $s: usize),+> TensorEmpty for $tensor<[T; SIZE], $($s),+, true>
        where
            [(); SIZE - 1]: Sized,                      // SIZE  >= 1
            [(); SIZE - shape_numel!([$($s)+])]: Sized, // SIZE  >= NUMEL
            [(); shape_numel!([$($s)+]) - SIZE]: Sized, // NUMEL >= SIZE
            $([(); $s - 1]: Sized),+                    // $s    >= 1
        {
            #[inline]
            fn empty() -> Self {
                let data: [T; SIZE] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

                Self::new(data)
            }
        }
    };
}
