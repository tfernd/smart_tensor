use super::*;
use crate::Data;

macro_rules! impl_data {
    ( $data:ident $($l:lifetime)? ) => {
        impl<$($l,)? T, const NUMEL: usize> Data for $data<$($l,)? T, NUMEL> {
            type Type = T;
            const NUMEL: usize = NUMEL;
        }
    };
}

impl_data!(StackData);
impl_data!(HeapData);
impl_data!(ViewData 'a);
impl_data!(ViewMutData 'a);
