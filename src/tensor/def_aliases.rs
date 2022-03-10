use super::*;

macro_rules! def_aliases {
    ( $alias:ident, $data:ident, $tensor:ident, [$($s:tt)+] ) => {
        pub type $alias<T, $(const $s: usize),+> = $tensor<StackData<T, {shape_numel!([$($s)+])}>, $($s),+>;
    };
}

def_aliases!(StackTensor1, StackData, Tensor1, [S0]);
def_aliases!(StackTensor2, StackData, Tensor2, [S0 S1]);
def_aliases!(StackTensor3, StackData, Tensor3, [S0 S1 S2]);
def_aliases!(StackTensor4, StackData, Tensor4, [S0 S1 S2 S3]);

def_aliases!(HeapTensor1, HeapData, Tensor1, [S0]);
def_aliases!(HeapTensor2, HeapData, Tensor2, [S0 S1]);
def_aliases!(HeapTensor3, HeapData, Tensor3, [S0 S1 S2]);
def_aliases!(HeapTensor4, HeapData, Tensor4, [S0 S1 S2 S3]);
