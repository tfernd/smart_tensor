use crate::traits::*;
use crate::Stride;

#[macro_use]
mod macros;

make_tensor!(Tensor1, 1, [S0]);
make_tensor!(Tensor2, 2, [S0 S1]);
make_tensor!(Tensor3, 3, [S0 S1 S3]);
make_tensor!(Tensor4, 4, [S0 S1 S3 S4]);

def_attrs!(Tensor1, 1, [S0]);
def_attrs!(Tensor2, 2, [S0 S1]);
def_attrs!(Tensor3, 3, [S0 S1 S3]);
def_attrs!(Tensor4, 4, [S0 S1 S3 S4]);

def_new!(Tensor1, 1, [S0]);
def_new!(Tensor2, 2, [S0 S1]);
def_new!(Tensor3, 3, [S0 S1 S3]);
def_new!(Tensor4, 4, [S0 S1 S3 S4]);
