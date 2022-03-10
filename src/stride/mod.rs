mod def_all;

#[derive(Debug, Clone)]
pub struct Stride<const DIM: usize>([isize; DIM]);
