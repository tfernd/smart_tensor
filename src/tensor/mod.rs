use crate::stride::Stride;
use crate::traits::{Data, TensorAttributes, TensorEmpty, TensorFull};

#[macro_use]
mod macros;

make_tensor!(Tensor1, 1, [S0]);
make_tensor!(Tensor2, 2, [S0 S1]);
make_tensor!(Tensor3, 3, [S0 S1 S3]);
make_tensor!(Tensor4, 4, [S0 S1 S3 S4]);

impl_attrs!(Tensor1, 1, [S0]);
impl_attrs!(Tensor2, 2, [S0 S1]);
impl_attrs!(Tensor3, 3, [S0 S1 S3]);
impl_attrs!(Tensor4, 4, [S0 S1 S3 S4]);

impl_new!(Tensor1, 1, [S0]);
impl_new!(Tensor2, 2, [S0 S1]);
impl_new!(Tensor3, 3, [S0 S1 S3]);
impl_new!(Tensor4, 4, [S0 S1 S3 S4]);

impl_empty!(Tensor1, 1, [S0]);
impl_empty!(Tensor2, 2, [S0 S1]);
impl_empty!(Tensor3, 3, [S0 S1 S3]);
impl_empty!(Tensor4, 4, [S0 S1 S3 S4]);

impl_full!(Tensor1, 1, [S0]);
impl_full!(Tensor2, 2, [S0 S1]);
impl_full!(Tensor3, 3, [S0 S1 S3]);
impl_full!(Tensor4, 4, [S0 S1 S3 S4]);

make_aliases!(@heap TensorHeap1 Tensor1, [S0]);
make_aliases!(@heap TensorHeap2 Tensor2, [S0 S1]);
make_aliases!(@heap TensorHeap3 Tensor3, [S0 S1 S2]);
make_aliases!(@heap TensorHeap4 Tensor4, [S0 S1 S2 S3]);

make_aliases!(@stack TensorStack1 Tensor1, [S0]);
make_aliases!(@stack TensorStack2 Tensor2, [S0 S1]);
make_aliases!(@stack TensorStack3 Tensor3, [S0 S1 S2]);
make_aliases!(@stack TensorStack4 Tensor4, [S0 S1 S2 S3]);
