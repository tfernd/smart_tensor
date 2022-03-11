use num_traits::{One, Zero};

/// A marker for structures that holds data.
pub trait DataTrait: Sized {
    /// The data structure inner type.
    type Type;

    /// The number of elements in the data.
    const NUMEL: usize;
}

/// A marker for creating of empty data.
pub trait DataEmptyTrait: DataTrait {
    /// Creates an empty data.
    fn empty() -> Self;
}

pub trait DataFullTrait: DataTrait
where
    Self::Type: Copy + Zero + One,
{
    /// Creates a data structure filled with `value`.
    fn full(value: Self::Type) -> Self;

    /// Creates a data structure filled with zeros.
    #[inline]
    fn zeros() -> Self {
        Self::full(Self::Type::zero())
    }

    /// Creates a data structure filled with ones.
    #[inline]
    fn ones() -> Self {
        Self::full(Self::Type::one())
    }
}
