use num_traits::{One, Zero};

pub trait TensorFull<T>
where
    Self: Sized,
    T: Copy,
{
    fn full(value: T) -> Self;

    #[inline]
    fn ones() -> Self
    where
        T: One,
    {
        Self::full(T::one())
    }

    #[inline]
    fn zeros() -> Self
    where
        T: Zero,
    {
        Self::full(T::zero())
    }
}
