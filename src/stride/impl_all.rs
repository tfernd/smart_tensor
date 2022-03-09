use super::Stride;

impl<const DIM: usize> Stride<DIM>
where
    [(); DIM - 1]: Sized, // D >= 1
{
    /// Create a new Stride.
    pub fn new(stride: [isize; DIM]) -> Self {
        Self(stride)
    }

    /// Create default strides from a given `shape`.
    pub fn default_stride(shape: [usize; DIM]) -> Self {
        let mut stride = [1; DIM];

        for i in (0..DIM - 1).rev() {
            stride[i] = stride[i + 1] * shape[i + 1] as isize
        }

        Self::new(stride)
    }
}
