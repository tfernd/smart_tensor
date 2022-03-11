use num_traits::{One, Zero};

use super::*;
use crate::traits::FullLikeData;

// TODO avoid repetition using macros

impl<T, const NUMEL: usize> FullLikeData for StackData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    type Output = StackData<T, NUMEL>;
}

impl<T, const NUMEL: usize> FullLikeData for HeapData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    type Output = HeapData<T, NUMEL>;
}

impl<'a, T, const NUMEL: usize> FullLikeData for SliceData<'a, T, NUMEL, true>
where
    T: Copy + Zero + One,
{
    type Output = StackData<T, NUMEL>;
}

impl<'a, T, const NUMEL: usize> FullLikeData for SliceData<'a, T, NUMEL, false>
where
    T: Copy + Zero + One,
{
    type Output = HeapData<T, NUMEL>;
}

impl<'a, T, const NUMEL: usize> FullLikeData for MutSliceData<'a, T, NUMEL, true>
where
    T: Copy + Zero + One,
{
    type Output = StackData<T, NUMEL>;
}

impl<'a, T, const NUMEL: usize> FullLikeData for MutSliceData<'a, T, NUMEL, false>
where
    T: Copy + Zero + One,
{
    type Output = HeapData<T, NUMEL>;
}
