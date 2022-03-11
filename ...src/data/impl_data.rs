use super::*;

impl<T, const NUMEL: usize> Data for StackData<T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}

impl<T, const NUMEL: usize> Data for HeapData<T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}

impl<'a, T, const NUMEL: usize> Data for SliceData<'a, T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}

impl<'a, T, const NUMEL: usize> Data for MutSliceData<'a, T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}
