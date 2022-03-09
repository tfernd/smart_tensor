use super::*;

// TODO move this to a separate file
/// Number of elements in a a shape.
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}

macro_rules! make_aliases {
    ( @heap $tensor_heap:ident $tensor:ident, [$($s:tt)+] ) => {
        /// An n-dimensional tensor allocated on the heap.
        pub type $tensor_heap<T, $(const $s: usize),+, const CANNONICAL: bool=true> = $tensor<Vec<T>, $($s),+, CANNONICAL>;
    };

    ( @stack $tensor_stack:ident $tensor:ident, [$($s:tt)+] ) => {
        /// An n-dimensional tensor allocated on the stack.
        pub type $tensor_stack<T, $(const $s: usize),+, const CANNONICAL: bool=true> = $tensor<[T; shape_numel!([$($s)+])], $($s),+, CANNONICAL>;
    };
}

// TODO move this one level up
make_aliases!(@heap TensorHeap1 Tensor1, [S0]);
make_aliases!(@heap TensorHeap2 Tensor2, [S0 S1]);
make_aliases!(@heap TensorHeap3 Tensor3, [S0 S1 S2]);
make_aliases!(@heap TensorHeap4 Tensor4, [S0 S1 S2 S3]);

// TODO move this one level up
make_aliases!(@stack TensorStack1 Tensor1, [S0]);
make_aliases!(@stack TensorStack2 Tensor2, [S0 S1]);
make_aliases!(@stack TensorStack3 Tensor3, [S0 S1 S2]);
make_aliases!(@stack TensorStack4 Tensor4, [S0 S1 S2 S3]);
