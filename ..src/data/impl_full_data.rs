use num_traits::{One, Zero};

use super::*;
use crate::traits::FullData;

macro_rules! impl_full_data {
    ( $data:ident $($vec:ident)? ) => {
        impl<T, const NUMEL: usize> FullData for $data<T, NUMEL>
        where
            T: Copy + Zero + One,
        {
            #[inline]
            fn full(value: T) -> Self {
                Self($($vec!)? [value; NUMEL])
            }
        }
    };
}

impl_full_data!(StackData);
impl_full_data!(HeapData vec);
