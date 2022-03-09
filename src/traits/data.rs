/// A trait to mark data types allowed inside a tensor.
pub trait Data {}

impl<T> Data for Vec<T> {}

impl<T> Data for &[T] {}

impl<T> Data for &mut [T] {}

impl<T, const DIM: usize> Data for [T; DIM] where [(); DIM - 1]: Sized {} // DIM >= 1
