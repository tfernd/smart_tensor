use super::*;
use crate::traits::DataTrait;

impl<T, const NUMEL: usize> DataTrait for StackData<T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}

impl<T, const NUMEL: usize> DataTrait for HeapData<T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}

impl<'a, T, const NUMEL: usize> DataTrait for SliceData<'a, T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}

impl<'a, T, const NUMEL: usize> DataTrait for MutSliceData<'a, T, NUMEL> {
    type Type = T;

    const NUMEL: usize = NUMEL;
}
