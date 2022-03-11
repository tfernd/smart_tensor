use num_traits::{One, Zero};

use super::*;

impl<T, const NUMEL: usize> DataFull for StackData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    fn full(value: T) -> Self {
        Self([value; NUMEL])
    }
}

impl<T, const NUMEL: usize> DataFull for HeapData<T, NUMEL>
where
    T: Copy + Zero + One,
{
    fn full(value: T) -> Self {
        Self(vec![value; NUMEL])
    }
}
