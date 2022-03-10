/// Number of elements in a a shape.
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}
