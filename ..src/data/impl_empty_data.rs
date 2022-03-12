use super::*;
use crate::traits::EmptyData;

impl<T, const NUMEL: usize> EmptyData for StackData<T, NUMEL> {
    #[inline]
    fn empty() -> Self {
        let data = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        Self(data)
    }
}

impl<T, const NUMEL: usize> EmptyData for HeapData<T, NUMEL> {
    #[inline]
    fn empty() -> Self {
        let mut data = Vec::with_capacity(NUMEL);
        unsafe { data.set_len(NUMEL) };

        Self(data)
    }
}
