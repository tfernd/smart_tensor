use super::*;

macro_rules! impl_empty {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        // Owned heap data.
        impl<T, $(const $s: usize),+> $tensor<Vec<T>, $($s),+, true>
        where
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            pub fn empty() -> Self {
                let mut data = Vec::with_capacity(Self::NUMEL);
                unsafe { data.set_len(Self::NUMEL) };

                Self::new(data)
            }
        }

        // Owned stack data.
        impl<T, const SIZE: usize, $(const $s: usize),+> $tensor<[T; SIZE], $($s),+, true>
        where
            // TODO check if SIZE = NUMEL
            // [(); SIZE - Self::NUMEL]: Sized,
            [(); SIZE - 1]: Sized, // D >= 1
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            pub fn empty() -> Self {
                let data: [T; SIZE] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

                Self::new(data)
            }
        }
    };
}

// TODO move this one level up
impl_empty!(Tensor1, 1, [S0]);
impl_empty!(Tensor2, 2, [S0 S1]);
impl_empty!(Tensor3, 3, [S0 S1 S2]);
impl_empty!(Tensor4, 4, [S0 S1 S2 S3]);
