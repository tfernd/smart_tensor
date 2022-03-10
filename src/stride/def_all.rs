use super::Stride;

impl<const DIM: usize> Stride<DIM> {
    pub fn new(stride: [isize; DIM]) -> Self {
        Self(stride)
    }

    pub fn default_stride(shape: [usize; DIM]) -> Self {
        let mut stride = [1; DIM];

        for i in (0..DIM - 1).rev() {
            stride[i] = stride[i + 1] * shape[i + 1] as isize
        }

        Self::new(stride)
    }
}
