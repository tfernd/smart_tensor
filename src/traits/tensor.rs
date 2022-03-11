use num_traits::{One, Zero};

use super::{DataEmptyTrait, DataFullTrait, DataTrait};

/// A marker for tensor.
pub trait TensorTrait<const DIM: usize>: Sized
where
    Self::Data: DataTrait,
{
    /// The data structure type.
    type Data;

    const SHAPE: [usize; DIM];

    /// Creates a new tensor from `data`.
    unsafe fn new(data: Self::Data) -> Self;
}

/// A marker to create a tensor with empty data.
pub trait TensorEmptyTrait<const DIM: usize>: TensorTrait<DIM>
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

/// A marker to create a tensor filled with a given value.
pub trait TensorFullTrait<const DIM: usize>: TensorTrait<DIM>
where
    Self::Data: DataFullTrait,
    // ? It is possible to avoid this madness?
    <<Self as TensorTrait<DIM>>::Data as DataTrait>::Type: Copy + Zero + One,
{
    /// Creates a data structure filled with `value`.
    fn full(value: <<Self as TensorTrait<DIM>>::Data as DataTrait>::Type) -> Self {
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
