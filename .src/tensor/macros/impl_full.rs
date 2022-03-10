macro_rules! impl_full {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        // Owned heap data.
        impl<T, $(const $s: usize),+> TensorFull<T> for $tensor<Vec<T>, $($s),+, true>
        where
            T: Copy,
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            fn full(value: T) -> Self {
                let data = vec![value; Self::NUMEL];

                Self::new(data)
            }
        }

        // Owned stack data.
        impl<T, const SIZE: usize, $(const $s: usize),+> TensorFull<T> for $tensor<[T; SIZE], $($s),+, true>
        where
            T: Copy,
            [(); SIZE - 1]: Sized,                      // SIZE  >= 1
            [(); SIZE - shape_numel!([$($s)+])]: Sized, // SIZE  >= NUMEL
            [(); shape_numel!([$($s)+]) - SIZE]: Sized, // NUMEL >= SIZE
            $([(); $s - 1]: Sized),+                    // $s    >= 1
        {
            #[inline]
            fn full(value: T) -> Self {
                let data = [value; SIZE];

                Self::new(data)
            }
        }
    };
}
