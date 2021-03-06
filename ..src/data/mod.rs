mod impl_as_mut_ref;
mod impl_as_ref;
mod impl_data;
mod impl_empty_data;
mod impl_full_data;
mod impl_full_like_data;

/// Data structure that owns data on the stack.
#[derive(Debug)]
pub struct StackData<T, const NUMEL: usize>(pub(crate) [T; NUMEL]);

/// Data structure that owns data on the heap.
#[derive(Debug)]
pub struct HeapData<T, const NUMEL: usize>(pub(crate) Vec<T>);

/// Data structure that references immutably data.
#[derive(Debug)]
pub struct RefData<'a, T, const NUMEL: usize, const STACK: bool>(pub(crate) &'a [T]);

/// Data structure that references mutably data.
#[derive(Debug)]
pub struct MutRefData<'a, T, const NUMEL: usize, const STACK: bool>(pub(crate) &'a mut [T]);
