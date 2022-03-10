// ? Add const SIZE?
pub trait Data {
    type Type;

    fn len(&self) -> usize;
}

/* -------------------------------------------------------------------------- */
/*                               implementations                              */
/* -------------------------------------------------------------------------- */

impl<T> Data for Vec<T> {
    type Type = T;

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

impl<T, const SIZE: usize> Data for [T; SIZE] {
    type Type = T;

    #[inline]
    fn len(&self) -> usize {
        SIZE
    }
}
