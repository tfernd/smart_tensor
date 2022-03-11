use num_traits::{One, Zero};

/// A marker for structures that contains array elements with fixed size.
pub trait Data: Sized {
    /// The data type of the elements in data.
    type Item;

    /// The number of elements in the data.
    const NUMEL: usize;
}

/// A marker for creating empty (unitialized) data.
pub trait EmptyData: Data {
    /// Creates empty (unitialized) data.
    fn empty() -> Self;
}

/// A marker for creating data filled with a given value.
pub trait FullData: Data
where
    Self::Item: Copy + Zero + One,
{
    /// Creates a data structure filled with `value`.
    fn full(value: Self::Item) -> Self;

    /// Creates a data structure filled with zeros.
    #[inline]
    fn zeros() -> Self {
        Self::full(Self::Item::zero())
    }

    /// Creates a data structure filled with ones.
    #[inline]
    fn ones() -> Self {
        Self::full(Self::Item::one())
    }
}

/// A marker for creating data filled with a given value
/// with the same container type as the data.
pub trait FullLikeData: Data
where
    Self::Output: FullData<Item = Self::Item>,
    Self::Item: Copy + Zero + One,
{
    type Output;

    /// Creates a data structure filled with `value` with same length.
    fn full_like(&self, value: Self::Item) -> Self::Output {
        Self::Output::full(value)
    }

    #[inline]
    fn zeros_like(&self) -> Self::Output {
        self.full_like(Self::Item::zero())
    }

    #[inline]
    fn ones_like(&self) -> Self::Output {
        self.full_like(Self::Item::one())
    }
}
