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
