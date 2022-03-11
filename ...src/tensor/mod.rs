use crate::*;

macro_rules! make_tensor {
    ( $tensor:ident, [$($s:tt)+]) => {
        struct $tensor<D: Data, $(const $s: usize),+> {
            data: D,
        }

        impl<D: Data, $(const $s: usize),+> Tensor for $tensor<D, $($s),+> {
            type Data = D;
            type Type = D::Type;

            const NUMEL: usize = shape_numel!([$($s)+]);
        }

        impl<D: DataEmpty, $(const $s: usize),+> TensorEmpty for $tensor<D, $($s),+> {
            fn empty() -> Self {
                $tensor {
                    data: D::empty(),
                }
            }
        }
    };
}

make_tensor!(Tensor1, [S0]);
make_tensor!(Tensor2, [S0 S1]);
make_tensor!(Tensor3, [S0 S1 S2]);
make_tensor!(Tensor4, [S0 S1 S2 S3]);
