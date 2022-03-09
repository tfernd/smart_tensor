mod impl_all; // TODO rename

/// Tensor strides (`DIM` >= 1).
#[derive(Debug, Clone)]
pub struct Stride<const DIM: usize>([isize; DIM])
where
    [(); DIM - 1]: Sized; // D >= 1
