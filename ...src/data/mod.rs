mod impl_data;
mod impl_data_empty;
mod impl_data_full;

use crate::*;

pub struct StackData<T, const NUMEL: usize>([T; NUMEL]);
pub struct HeapData<T, const NUMEL: usize>(Vec<T>);
pub struct SliceData<'a, T, const NUMEL: usize>(&'a [T]);
pub struct MutSliceData<'a, T, const NUMEL: usize>(&'a mut [T]);
