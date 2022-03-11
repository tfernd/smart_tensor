/// Number of elements in a a shape.
#[doc(hidden)]
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}
