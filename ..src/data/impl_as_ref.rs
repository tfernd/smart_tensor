use super::*;
use crate::traits::{AsMutRefData, AsRefData, Data};

macro_rules! impl_as_ref {
    ( $data:ident $(,$s:literal)? $(;$stack:tt $l:lifetime)? ) => {
        impl<'a, T, const NUMEL: usize $(,const $stack: bool)?> AsRefData<'a> for $data<$($l,)? T, NUMEL $(,$stack)?>
        where
            T: 'a,
        {
            type Output = RefData<'a, T, NUMEL $(,$s)? $(,$stack)?>;

            #[inline]
            fn as_ref_data(&'a self) -> Self::Output {
                RefData(&self.0)
            }
        }
    };
}

impl_as_ref!(StackData, true);
impl_as_ref!(HeapData, false);
impl_as_ref!(RefData; STACK 'a);
