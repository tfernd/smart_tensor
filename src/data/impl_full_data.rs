use num_traits::{One, Zero};

use super::*;
use crate::traits::FullData;

impl<T, const NUMEL: usize> FullData for StackData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    #[inline]
    fn full(value: T) -> Self {
        Self([value; NUMEL])
    }
}

impl<T, const NUMEL: usize> FullData for HeapData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    #[inline]
    fn full(value: T) -> Self {
        Self(vec![value; NUMEL])
    }
}
