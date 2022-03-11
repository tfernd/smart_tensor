mod impl_data;
mod impl_empty_data;
mod impl_full_data;
mod impl_full_like_data;

/// Data structure that owns data on the stack.
#[derive(Debug)]
pub struct StackData<T, const NUMEL: usize>([T; NUMEL]);

/// Data structure that owns data on the heap.
#[derive(Debug)]
pub struct HeapData<T, const NUMEL: usize>(Vec<T>);

/// Data structure that references immutably data.
#[derive(Debug)]
pub struct SliceData<'a, T, const NUMEL: usize, const STACK: bool>(&'a [T]);

/// Data structure that references mutably data.
#[derive(Debug)]
pub struct MutSliceData<'a, T, const NUMEL: usize, const STACK: bool>(&'a mut [T]);
