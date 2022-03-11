mod impl_data_empty;
mod impl_data_full;
mod impl_data_trait;

#[derive(Debug)]
pub struct StackData<T, const NUMEL: usize>([T; NUMEL]);

#[derive(Debug)]
pub struct HeapData<T, const NUMEL: usize>(Vec<T>);

#[derive(Debug)]
pub struct SliceData<'a, T, const NUMEL: usize>(&'a [T]);

#[derive(Debug)]
pub struct MutSliceData<'a, T, const NUMEL: usize>(&'a mut [T]);
