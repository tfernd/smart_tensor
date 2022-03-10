use crate::Stride;

pub trait TensorAttributes<const DIM: usize> {
    const DIM: usize = DIM;
    const SHAPE: [usize; DIM];
    const NUMEL: usize;

    fn stride(&self) -> Stride<{ Self::DIM }>;
    fn is_cannonical(&self) -> bool;

    #[inline]
    fn shape(&self) -> [usize; DIM] {
        Self::SHAPE
    }

    #[inline]
    fn numel(&self) -> usize {
        Self::NUMEL
    }

    #[inline]
    fn dim(&self) -> usize {
        Self::DIM
    }
}
