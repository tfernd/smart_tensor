use crate::tensor::{Tensor1, Tensor2, Tensor3, Tensor4};
use crate::traits::Data;

// TODO put this in another place... to avoid copy...
/// Number of elements in a a shape.
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}

macro_rules! make_views {
    ( $trait:ident, $fun:ident, $tensor:ident, [$($v:tt)+] $(, $mut:tt)? ) => {
        pub trait $trait<T, const SIZE: usize>
        where
            [(); SIZE - 1]: Sized,
        {
            fn $fun<$(const $v: usize),+>(& $($mut)? self) -> $tensor<& $($mut)? [T], $($v),+, true>
            where
                $([(); $v - 1]: Sized,)+                    // $v    >= 1
                [(); SIZE - shape_numel!([$($v)+])]: Sized, // SIZE  >= NUMEL;
                [(); shape_numel!([$($v)+]) - SIZE]: Sized; // NUMEL >= SIZE;
        }

    };
}

make_views!(View1, view1, Tensor1, [V0]);
make_views!(View2, view2, Tensor2, [V0 V1]);
make_views!(View3, view3, Tensor3, [V0 V1 V2]);
make_views!(View4, view4, Tensor4, [V0 V1 V2 V3]);

make_views!(MutView1, mut_view1, Tensor1, [V0], mut);
make_views!(MutView2, mut_view2, Tensor2, [V0 V1]);
make_views!(MutView3, mut_view3, Tensor3, [V0 V1 V2]);
make_views!(MutView4, mut_view4, Tensor4, [V0 V1 V2 V3]);
