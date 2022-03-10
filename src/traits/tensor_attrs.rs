use crate::Stride;

pub trait TensorAttributes<const DIM: usize>
where
    [(); DIM - 1]: Sized,
{
    const SHAPE: [usize; DIM];
    const NUMEL: usize;
    const DIM: usize;

    fn stride(&self) -> Stride<DIM>;
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
        DIM
    }
}
