use super::*;
use crate::traits::Data;

impl<T, const NUMEL: usize> Data for StackData<T, NUMEL> {
    type Item = T;

    const NUMEL: usize = NUMEL;
}

impl<T, const NUMEL: usize> Data for HeapData<T, NUMEL> {
    type Item = T;

    const NUMEL: usize = NUMEL;
}

impl<'a, T, const NUMEL: usize, const STACK: bool> Data for SliceData<'a, T, NUMEL, STACK> {
    type Item = T;

    const NUMEL: usize = NUMEL;
}

impl<'a, T, const NUMEL: usize, const STACK: bool> Data for MutSliceData<'a, T, NUMEL, STACK> {
    type Item = T;

    const NUMEL: usize = NUMEL;
}
