use num_traits::{One, Zero};

use super::{Data, EmptyData, FullData, FullLikeData};

/// A marker for n-dimensional tensor with fixed-sized number of elements.
pub trait Tensor<const DIM: usize>: Sized
where
    Self::Container: Data<Item = Self::Item>,
{
    /// The data structure container.
    type Container;

    /// The data type of the elements in data.
    type Item;

    /// The shape of the tensor.
    const SHAPE: [usize; DIM]; // ? needed?

    /// Creates a new tensor from `data`.
    unsafe fn new_unchecked(data: Self::Container) -> Self; // TODO move to a new trait?
}

/// A marker for creating of an empty (unitialized) tensor.
pub trait EmptyTensor<const DIM: usize>: Tensor<DIM>
where
    Self::Container: EmptyData<Item = Self::Item>,
{
    /// Creates an empty (unitialized) tensor.
    #[inline]
    fn empty() -> Self {
        let data = Self::Container::empty();
        unsafe { Self::new_unchecked(data) }
    }
}

/// A marker for creating a tensor filled with a given value.
pub trait FullTensor<const DIM: usize>: Tensor<DIM>
where
    Self::Container: FullData<Item = Self::Item>,
    Self::Item: Copy + Zero + One,
{
    /// Creates a tensor filled with `value`.
    fn full(value: Self::Item) -> Self {
        let data = Self::Container::full(value);
        unsafe { Self::new_unchecked(data) }
    }

    /// Creates a tensor filled with zeros.
    #[inline]
    fn zeros() -> Self {
        Self::full(Self::Item::zero())
    }

    /// Creates a tensor filled with ones.
    #[inline]
    fn ones() -> Self {
        Self::full(Self::Item::one())
    }
}

/// A marker for creating a tensor filled with a given value
/// with the same container type as the data.
pub trait FullLikeTensor<const DIM: usize>: Tensor<DIM>
where
    Self::Output: FullTensor<DIM, Item = Self::Item>,
    <Self::Output as Tensor<DIM>>::Container: FullData<Item = Self::Item>,
    Self::Item: Copy + Zero + One,
{
    /// The tensor output type.
    type Output;

    /// Creates a tensor filled with `value` with same shape.
    #[inline]
    fn full_like(&self, value: Self::Item) -> Self::Output {
        Self::Output::full(value)
    }

    /// Creates a tensor filled with zeros with same shape.
    #[inline]
    fn zeros_like(&self) -> Self::Output {
        self.full_like(Self::Item::zero())
    }

    /// Creates a tensor filled with ones with same shape.
    #[inline]
    fn ones_like(&self) -> Self::Output {
        self.full_like(Self::Item::one())
    }
}
