use num_traits::{One, Zero};

/// A marker for structures that contains array elements with fixed size.
pub trait Data: Sized {
    /// The data type of the elements in data.
    type Item;

    /// The number of elements in the data.
    const NUMEL: usize;

    // ? Add new?
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
    /// The data output type.
    type Output;

    /// Creates a data structure filled with `value` with same length.
    #[inline]
    fn full_like(&self, value: Self::Item) -> Self::Output {
        Self::Output::full(value)
    }

    /// Creates a data structure filled with zeros with same length.
    #[inline]
    fn zeros_like(&self) -> Self::Output {
        self.full_like(Self::Item::zero())
    }

    /// Creates a data structure filled with ones with same length.
    #[inline]
    fn ones_like(&self) -> Self::Output {
        self.full_like(Self::Item::one())
    }
}

/// A marker for creating imutable data references.
pub trait AsRefData<'a>: Data
where
    // ? lifetime bound needed?
    Self::Output: Data<Item = Self::Item>,
    Self::Item: 'a,
{
    /// The data output type.
    type Output;

    /// Creates a reference to the data.
    fn as_ref_data(&'a self) -> Self::Output;
}

/// A marker for creating mutable data references.
pub trait AsMutRefData<'a>: Data
where
    // ? lifetime bound needed?
    Self::Output: Data<Item = Self::Item>,
    Self::Item: 'a,
{
    /// The data output type.
    type Output;

    /// Creates a reference to the data.
    fn as_mut_ref_data(&'a mut self) -> Self::Output;
}
