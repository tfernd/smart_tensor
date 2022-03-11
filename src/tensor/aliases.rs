use super::*;
use crate::{HeapData, StackData};

macro_rules! make_aliases {
    ( $alias:ident, $tensor:ident, $data:ident, [$($s:tt)+] ) => {
        pub type $alias<T, $(const $s: usize),+> = $tensor<$data<T, {shape_numel!([$($s)+])}>, $($s),+>;
    };
}

make_aliases!(StackTensor1, Tensor1, StackData, [S0]);
make_aliases!(StackTensor2, Tensor2, StackData, [S0 S1]);
make_aliases!(StackTensor3, Tensor3, StackData, [S0 S1 S2]);
make_aliases!(StackTensor4, Tensor4, StackData, [S0 S1 S2 S3]);

make_aliases!(HeapTensor1, Tensor1, HeapData, [S0]);
make_aliases!(HeapTensor2, Tensor2, HeapData, [S0 S1]);
make_aliases!(HeapTensor3, Tensor3, HeapData, [S0 S1 S2]);
make_aliases!(HeapTensor4, Tensor4, HeapData, [S0 S1 S2 S3]);
