mod impl_data;

#[derive(Debug)]
pub struct StackData<T, const NUMEL: usize>(pub(crate) [T; NUMEL]);

#[derive(Debug)]
pub struct HeapData<T, const NUMEL: usize>(pub(crate) Vec<T>);

#[derive(Debug)]
pub struct ViewData<'a, T, const NUMEL: usize>(pub(crate) &'a [T]);

#[derive(Debug)]
pub struct ViewMutData<'a, T, const NUMEL: usize>(pub(crate) &'a mut [T]);
