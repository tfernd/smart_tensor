use num_traits::{One, Zero};

use super::*;
use crate::traits::FullLikeData;

macro_rules! impl_full_like {
    ($data:ident $output:ident $($bool:literal $l:lifetime)?) => {
        impl<$($l,)? T, const NUMEL: usize> FullLikeData for $data<$($l,)? T, NUMEL $(,$bool)?>
        where
            T: Copy + Zero + One,
        {
            type Output = $output<T, NUMEL>;
        }
    };
}

impl_full_like!(StackData StackData);
impl_full_like!(HeapData HeapData);

impl_full_like!(SliceData StackData true 'a);
impl_full_like!(SliceData HeapData false 'a);

impl_full_like!(MutSliceData StackData true 'a);
impl_full_like!(MutSliceData HeapData false 'a);
