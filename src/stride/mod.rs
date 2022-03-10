mod impl_all; // TODO rename

/// Tensor strides.
#[derive(Debug, Clone)]
pub struct Stride<const DIM: usize>([isize; DIM])
where
    [(); DIM - 1]: Sized; // DIM >= 1
