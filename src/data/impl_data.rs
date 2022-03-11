use super::*;
use crate::traits::Data;

macro_rules! impl_data {
    ( $data:ident $($l:lifetime $s:ident)? ) => {
        impl<$($l,)? T, const NUMEL: usize $(,const $s: bool)?> Data for $data<$($l,)? T, NUMEL $(,$s)?> {
            type Item = T;

            const NUMEL: usize = NUMEL;
        }
    };
}

impl_data!(StackData);
impl_data!(HeapData);
impl_data!(RefData 'a STACK);
impl_data!(MutRefData 'a STACK);
