use num_traits::{One, Zero};

use super::{DataEmptyTrait, DataFullTrait, DataTrait};

/// A marker for tensor.
pub trait TensorTrait: Sized
where
    Self::Data: DataTrait,
{
    /// The data structure type.
    type Data;

    /// Creates a new tensor from `data`.
    unsafe fn new(data: Self::Data) -> Self;
}

pub trait TensorEmptyTrait: TensorTrait
where
    Self::Data: DataEmptyTrait,
{
    /// Creates an empty tensor.
    #[inline]
    fn empty() -> Self {
        let data = Self::Data::empty();
        unsafe { Self::new(data) }
    }
}

pub trait TensorFullTrait: TensorTrait
where
    Self::Data: DataFullTrait,
    <<Self as TensorTrait>::Data as DataTrait>::Type: Copy + Zero + One,
{
    /// Creates a data structure filled with `value`.
    fn full(value: <<Self as TensorTrait>::Data as DataTrait>::Type) -> Self {
        let data = Self::Data::full(value);
        unsafe { Self::new(data) }
    }

    /// Creates a data structure filled with zeros.
    #[inline]
    fn zeros() -> Self {
        let data = Self::Data::zeros();
        unsafe { Self::new(data) }
    }

    /// Creates a data structure filled with ones.
    #[inline]
    fn ones() -> Self {
        let data = Self::Data::ones();
        unsafe { Self::new(data) }
    }
}
