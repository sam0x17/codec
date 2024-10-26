use core::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bytes([u8]);

impl Bytes {
    // To create a `&Bytes` from a `&[u8]`
    pub fn from_slice(slice: &[u8]) -> &Self {
        // Safety: `Bytes` is a transparent wrapper around `[u8]`
        unsafe { &*(slice as *const [u8] as *const Bytes) }
    }

    // To create a `&mut Bytes` from a `&mut [u8]`
    pub fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
        // Safety: `Bytes` is a transparent wrapper around `[u8]`
        unsafe { &mut *(slice as *mut [u8] as *mut Bytes) }
    }
}

// Implement Deref for `&Bytes`
impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement DerefMut for `&mut Bytes`
impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
