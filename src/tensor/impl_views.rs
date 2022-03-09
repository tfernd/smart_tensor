use super::*;

// TODO move this to a separate file
/// Number of elements in a a shape.
macro_rules! shape_numel {
    ( [$s:tt $($tail:tt)*] ) => { $s $(* $tail)* };
}

/// Create views from a tensor.
macro_rules! impl_sub_views {
    ( $data:ty, $tensor:ident, $view:ident, $tensor_view:ident, [$($s:tt)+], [$($v:tt)+] $({$fun:ident})? $([$numel:ident])? ) => {
        impl<T, $(const $s: usize),+ $(,const $numel: usize)?> $tensor<$data, $($s),+, true>
        where
            // TODO check NUMEL against shape-numel
            $([(); $numel - 1]: Sized,)?
            $([(); $s - 1]: Sized),+ // $s >= 1
        {
            #[inline]
            pub fn $view<$(const $v: usize),+>(&self) -> $tensor_view<&[T], $($v),+, true>
            where
                $([(); $v - 1]: Sized,)+                                      // $v >= 1
                [(); shape_numel!([$($v)+]) - shape_numel!([$($s)+])]: Sized, // numel(v) >= numel(s)
                [(); shape_numel!([$($s)+]) - shape_numel!([$($v)+])]: Sized, // numel(s) >= numel(v)
            {
                $tensor_view::new(self.data $(.$fun())?)
            }
        }
    };
}

macro_rules! impl_views {
    ( $tensor:ident, $dim:literal, [$($s:tt)+] ) => {
        impl_sub_views!(Vec<T>, $tensor, view1, Tensor1, [$($s)+], [V0         ] {as_slice});
        impl_sub_views!(Vec<T>, $tensor, view2, Tensor2, [$($s)+], [V0 V1      ] {as_slice});
        impl_sub_views!(Vec<T>, $tensor, view3, Tensor3, [$($s)+], [V0 V1 V2   ] {as_slice});
        impl_sub_views!(Vec<T>, $tensor, view4, Tensor4, [$($s)+], [V0 V1 V2 V3] {as_slice});

        impl_sub_views!([T; NUMEL], $tensor, view1, Tensor1, [$($s)+], [V0         ] {as_slice} [NUMEL]);
        impl_sub_views!([T; NUMEL], $tensor, view2, Tensor2, [$($s)+], [V0 V1      ] {as_slice} [NUMEL]);
        impl_sub_views!([T; NUMEL], $tensor, view3, Tensor3, [$($s)+], [V0 V1 V2   ] {as_slice} [NUMEL]);
        impl_sub_views!([T; NUMEL], $tensor, view4, Tensor4, [$($s)+], [V0 V1 V2 V3] {as_slice} [NUMEL]);

        impl_sub_views!(&[T], $tensor, view1, Tensor1, [$($s)+], [V0]);
        impl_sub_views!(&[T], $tensor, view2, Tensor2, [$($s)+], [V0 V1]);
        impl_sub_views!(&[T], $tensor, view3, Tensor3, [$($s)+], [V0 V1 V2]);
        impl_sub_views!(&[T], $tensor, view4, Tensor4, [$($s)+], [V0 V1 V2 V3]);
    };
}

// TODO move this one level up
impl_views!(Tensor1, 1, [S0]);
impl_views!(Tensor2, 2, [S0 S1]);
impl_views!(Tensor3, 3, [S0 S1 S2]);
impl_views!(Tensor4, 4, [S0 S1 S2 S3]);
