mod def_all;

/// The stride of a tensor.
#[derive(Debug, Clone)]
pub struct Stride<const DIM: usize>([isize; DIM]);
