use num_traits::{One, Zero};

use crate::traits::*;

// TODO create a macro to avoid repetition
// TODO create HeapData

/// A data container for elements on the stack.
#[derive(Debug)]
pub struct StackData<T, const N: usize>(pub(crate) [T; N]);

/* -------------------------------------------------------------------------- */
/*                               implement Data                               */
/* -------------------------------------------------------------------------- */

impl<T, const N: usize> Data for StackData<T, N> {
    type Item = T;
}

impl<T, const N: usize> Data for &StackData<T, N> {
    type Item = T;
}

impl<T, const N: usize> Data for &mut StackData<T, N> {
    type Item = T;
}

/* -------------------------------------------------------------------------- */
/*                           implement SameDataSize                           */
/* -------------------------------------------------------------------------- */

unsafe impl<T, const N: usize> SameDataSize<StackData<T, N>> for StackData<T, N> {}
unsafe impl<T, const N: usize> SameDataSize<StackData<T, N>> for &StackData<T, N> {}
unsafe impl<T, const N: usize> SameDataSize<&StackData<T, N>> for StackData<T, N> {}
unsafe impl<T, const N: usize> SameDataSize<&StackData<T, N>> for &StackData<T, N> {}
unsafe impl<T, const N: usize> SameDataSize<StackData<T, N>> for &mut StackData<T, N> {}
unsafe impl<T, const N: usize> SameDataSize<&mut StackData<T, N>> for StackData<T, N> {}
unsafe impl<T, const N: usize> SameDataSize<&mut StackData<T, N>> for &mut StackData<T, N> {}

/* -------------------------------------------------------------------------- */
/*                             implement EmptyData                            */
/* -------------------------------------------------------------------------- */

impl<T, const N: usize> EmptyData for StackData<T, N> {
    fn empty() -> Self {
        let data = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        Self(data)
    }
}

/* -------------------------------------------------------------------------- */
/*                           implement EmptyLikeData                          */
/* -------------------------------------------------------------------------- */

impl<T, const N: usize> EmptyLikeData for StackData<T, N> {
    type Output = StackData<T, N>;
}

/* -------------------------------------------------------------------------- */
/*                             implement FullData                             */
/* -------------------------------------------------------------------------- */

impl<T: Copy + Zero + One, const N: usize> FullData for StackData<T, N> {
    fn full(value: T) -> Self {
        Self([value; N])
    }
}

/* -------------------------------------------------------------------------- */
/*                           implement FullLikeData                           */
/* -------------------------------------------------------------------------- */

impl<T: Copy + Zero + One, const N: usize> FullLikeData for StackData<T, N> {
    type Output = StackData<T, N>;
}

impl<T: Copy + Zero + One, const N: usize> FullLikeData for &StackData<T, N> {
    type Output = StackData<T, N>;
}

impl<T: Copy + Zero + One, const N: usize> FullLikeData for &mut StackData<T, N> {
    type Output = StackData<T, N>;
}
