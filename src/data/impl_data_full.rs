use num_traits::{One, Zero};

use super::*;
use crate::traits::DataFullTrait;

impl<T, const NUMEL: usize> DataFullTrait for StackData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    #[inline]
    fn full(value: T) -> Self {
        Self([value; NUMEL])
    }
}

impl<T, const NUMEL: usize> DataFullTrait for HeapData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    #[inline]
    fn full(value: T) -> Self {
        Self(vec![value; NUMEL])
    }
}
