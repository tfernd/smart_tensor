use num_traits::{One, Zero};

/// A trait for a data containers.
pub trait Data: Sized {
    /// The type of the elements in the container.
    type Item;
}

/// A trait for data containers of the same size.
///
/// **Implementations need to make sure they indeed have the same size!**
pub unsafe trait SameDataSize<T>
where
    Self: Data,
    T: Data<Item = Self::Item>,
{
}

/// A trait for creating data containers with unitialized elements.
pub trait EmptyData: Data {
    /// Creates a data container with unitialized elements.
    fn empty() -> Self;
}

/// A trait for creating data containers with unitialized elements
/// based on `self`.
pub trait EmptyLikeData: Data
where
    Self::Output: EmptyData<Item = Self::Item>,
    Self: SameDataSize<Self::Output>,
{
    /// The type of the data container output.
    type Output;

    /// Creates a data container with unitialized elements
    /// based on `self`.
    #[inline]
    fn empty_like(&self) -> Self::Output {
        Self::Output::empty()
    }
}

/// A trait for creating data containers filled with a given value.
pub trait FullData: Data
where
    Self::Item: Copy + Zero + One,
{
    /// Creates a data container filled with a given value.
    fn full(value: Self::Item) -> Self;

    /// Creates a data container filled with zeros.
    #[inline]
    fn zeros() -> Self {
        Self::full(Self::Item::zero())
    }

    /// Creates a data container filled with ones.
    #[inline]
    fn ones() -> Self {
        Self::full(Self::Item::one())
    }
}

/// A trait for creating data containers filled with a given value
/// based on `self`.
pub trait FullLikeData: Data
where
    Self::Item: Copy + Zero + One,
    Self::Output: FullData<Item = Self::Item>,
    Self: SameDataSize<Self::Output>,
{
    /// The type of the data container output.
    type Output;

    /// Creates a data container filled with a given value
    /// based on `self`.
    #[inline]
    fn full_like(&self, value: Self::Item) -> Self::Output {
        Self::Output::full(value)
    }

    /// Creates a data container filled with zeros
    /// based on `self`.
    #[inline]
    fn zeros_like(&self) -> Self::Output {
        self.full_like(Self::Item::zero())
    }

    /// Creates a data container filled with ones
    /// based on `self`.
    #[inline]
    fn ones_like(&self) -> Self::Output {
        self.full_like(Self::Item::one())
    }
}
