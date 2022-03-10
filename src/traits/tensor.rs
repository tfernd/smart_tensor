pub trait Tensor<const DIM: usize> {
    type Data;
    type Type;

    const NUMEL: usize;
    const SHAPE: [usize; DIM];
}
