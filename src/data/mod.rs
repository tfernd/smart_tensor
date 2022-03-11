mod impl_data_empty;
mod impl_data_full;
mod impl_data_trait;

/// Data structure that owns data on the stack.
#[derive(Debug)]
pub struct StackData<T, const NUMEL: usize>([T; NUMEL]);

/// Data structure that owns data on the heap.
#[derive(Debug)]
pub struct HeapData<T, const NUMEL: usize>(Vec<T>);

/// Data structure that references immutably data.
#[derive(Debug)]
pub struct SliceData<'a, T, const NUMEL: usize>(&'a [T]);

/// Data structure that references mutably data.
#[derive(Debug)]
pub struct MutSliceData<'a, T, const NUMEL: usize>(&'a mut [T]);
