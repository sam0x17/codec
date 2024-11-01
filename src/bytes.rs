mod byte_array;
mod byte_slice;
mod byte_vec;

use core::ops::RangeBounds;

pub use byte_array::*;
pub use byte_slice::*;
pub use byte_vec::*;

pub trait Bytes {
    fn as_bytes(&self) -> &ByteSlice;
    fn as_slice(&self) -> &[u8];
    fn get(&self, index: usize) -> Option<u8>;
    fn get_mut(&mut self, index: usize) -> Option<&mut u8>;
    fn get_range(&self, range: impl RangeBounds<usize>) -> Option<&ByteSlice>;
    fn get_range_mut(&mut self, range: impl RangeBounds<usize>) -> Option<&mut ByteSlice>;
    fn len(&self) -> usize;
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn split_at(&self, mid: usize) -> (&ByteSlice, &ByteSlice);
    fn split_at_mut(&mut self, mid: usize) -> (&mut ByteSlice, &mut ByteSlice);
    fn to_vec(&self) -> ByteVec;
}

// TODO: get rid of deref impls and make everything non-panicking
