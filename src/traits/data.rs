/// A trait to mark data types allowed inside a tensor.
pub trait Data {
    /// Returns the number of elements the data structure holds.
    fn len(&self) -> usize;
}

impl<T> Data for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Data for &[T] {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T> Data for &mut [T] {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T, const SIZE: usize> Data for [T; SIZE]
where
    [(); SIZE - 1]: Sized, // SIZE >= 1
{
    #[inline]
    fn len(&self) -> usize {
        SIZE
    }
}
