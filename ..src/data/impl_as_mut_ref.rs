use super::*;
use crate::traits::{AsMutRefData, AsRefData, Data};

macro_rules! impl_as_mut_ref {
    ( $data:ident $s:literal ) => {
        impl<'a, T, const NUMEL: usize> AsMutRefData<'a> for $data<T, NUMEL>
        where
            T: 'a,
        {
            type Output = MutRefData<'a, T, NUMEL, $s>;

            #[inline]
            fn as_mut_ref_data(&'a mut self) -> Self::Output {
                MutRefData(&mut self.0)
            }
        }
    };
}

impl_as_mut_ref!(StackData true);
impl_as_mut_ref!(HeapData false);
