use super::*;

impl<T, const NUMEL: usize> DataEmpty for StackData<T, NUMEL> {
    fn empty() -> Self {
        let data = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        Self(data)
    }
}

impl<T, const NUMEL: usize> DataEmpty for HeapData<T, NUMEL> {
    fn empty() -> Self {
        let mut data = Vec::with_capacity(NUMEL);
        unsafe { data.set_len(NUMEL) };

        Self(data)
    }
}
