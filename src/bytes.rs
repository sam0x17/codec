mod byte_array;
mod byte_slice;
mod byte_vec;

pub use byte_array::*;
pub use byte_slice::*;
pub use byte_vec::*;

pub trait Bytes {
    fn as_bytes(&self) -> &ByteSlice;
    fn as_slice(&self) -> &[u8];
}

// TODO: get rid of deref impls and make everything non-panicking
