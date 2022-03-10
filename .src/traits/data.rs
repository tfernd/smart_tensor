/// A trait to mark data types allowed inside a tensor.
pub trait Data {
    type Type;

    /// Returns the number of elements the data structure holds.
    fn len(&self) -> usize;
}

impl<T> Data for Vec<T> {
    type Type = T;

    // ? A bit silly this...
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Data for &[T] {
    type Type = T;

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T> Data for &mut [T] {
    type Type = T;

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T, const SIZE: usize> Data for [T; SIZE]
where
    [(); SIZE - 1]: Sized, // SIZE >= 1
{
    type Type = T;

    #[inline]
    fn len(&self) -> usize {
        SIZE
    }
}
