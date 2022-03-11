use num_traits::{One, Zero};

use super::{Data, EmptyData, FullData, FullLikeData};

/// A marker for n-dimensional tensor with fixed-sized number of elements.
pub trait Tensor<const DIM: usize>: Sized
where
    Self::Data: Data,
{
    /// The data structure type.
    type Data;

    /// The shape of the tensor.
    const SHAPE: [usize; DIM]; // ? needed?

    /// Creates a new tensor from `data`.
    unsafe fn new(data: Self::Data) -> Self;
}

/// A marker for creating of an empty (unitialized) tensor.
pub trait EmptyTensor<const DIM: usize>: Tensor<DIM>
where
    Self::Data: EmptyData,
{
    /// Creates an empty (unitialized) tensor.
    #[inline]
    fn empty() -> Self {
        let data = Self::Data::empty();
        unsafe { Self::new(data) }
    }
}

/// A marker for creating a tensor filled with a given value.
pub trait FullTensor<const DIM: usize>: Tensor<DIM>
where
    Self::Data: FullData,
    <Self::Data as Data>::Item: Copy + Zero + One,
{
    /// Creates a tensor filled with `value`.
    fn full(value: <Self::Data as Data>::Item) -> Self {
        let data = Self::Data::full(value);
        unsafe { Self::new(data) }
    }

    /// Creates a tensor filled with zeros.
    #[inline]
    fn zeros() -> Self {
        let data = Self::Data::zeros();
        unsafe { Self::new(data) }
    }

    /// Creates a tensor filled with ones.
    #[inline]
    fn ones() -> Self {
        let data = Self::Data::ones();
        unsafe { Self::new(data) }
    }
}

// ? How to implement this?
// / A marker for creating a tensor filled with a given value
// / with the same container type as the data.
// pub trait FullLikeTensor<const DIM: usize>: Tensor<DIM> {}
