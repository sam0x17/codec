mod byte_array;
mod byte_slice;
mod byte_vec;

use core::{fmt::Debug, hash::Hash, slice::SliceIndex};

pub use byte_array::*;
pub use byte_slice::*;
pub use byte_vec::*;

pub trait Bytes: Debug + PartialEq + Eq + PartialOrd + Ord + Hash {
    fn as_bytes(&self) -> &ByteSlice;
    fn as_slice(&self) -> &[u8];
    fn get(&self, index: usize) -> Option<u8>;
    fn get_mut(&mut self, index: usize) -> Option<&mut u8>;
    fn range(&self, range: impl SliceIndex<[u8], Output = [u8]>) -> &ByteSlice;
    fn range_mut(&mut self, range: impl SliceIndex<[u8], Output = [u8]>) -> &mut ByteSlice;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn split_at(&self, mid: usize) -> (&ByteSlice, &ByteSlice);
    fn split_at_mut(&mut self, mid: usize) -> (&mut ByteSlice, &mut ByteSlice);
    fn to_vec(&self) -> ByteVec;
}
