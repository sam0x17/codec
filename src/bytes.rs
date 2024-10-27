use core::ops::{Deref, DerefMut, Range};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bytes([u8]);

impl Bytes {
    /// Create a new `Bytes` from a slice of bytes
    pub const fn from_slice(slice: &[u8]) -> &Self {
        // Safety: `Bytes` is a transparent wrapper around `[u8]`
        unsafe { &*(slice as *const [u8] as *const Bytes) }
    }

    /// Create a new `Bytes` from a mutable slice of bytes
    pub fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
        // Safety: `Bytes` is a transparent wrapper around `[u8]`
        unsafe { &mut *(slice as *mut [u8] as *mut Bytes) }
    }

    pub const fn split_at(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = self.0.split_at(mid);
        (Bytes::from_slice(left), Bytes::from_slice(right))
    }

    pub const fn split_at_checked(&self, mid: usize) -> Option<(&Self, &Self)> {
        match self.0.split_at_checked(mid) {
            Some((left, right)) => Some((Bytes::from_slice(left), Bytes::from_slice(right))),
            None => None,
        }
    }

    pub const fn split_at_unchecked(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = unsafe { self.0.split_at_unchecked(mid) };
        (Bytes::from_slice(left), Bytes::from_slice(right))
    }

    pub const fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    pub const fn trim_ascii_start(&self) -> &Self {
        Bytes::from_slice(self.0.trim_ascii_start())
    }

    pub const fn trim_ascii_end(&self) -> &Self {
        Bytes::from_slice(self.0.trim_ascii_end())
    }

    pub const fn trim_ascii(&self) -> &Self {
        Bytes::from_slice(self.0.trim_ascii())
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn first(&self) -> Option<&u8> {
        self.0.first()
    }

    pub const fn last(&self) -> Option<&u8> {
        self.0.last()
    }

    pub const fn split_first(&self) -> Option<(&u8, &Self)> {
        match self.0.split_first() {
            Some((first, rest)) => Some((first, Bytes::from_slice(rest))),
            None => None,
        }
    }

    pub const fn split_last(&self) -> Option<(&u8, &Self)> {
        match self.0.split_last() {
            Some((last, rest)) => Some((last, Bytes::from_slice(rest))),
            None => None,
        }
    }

    pub const fn first_chunk<const N: usize>(&self) -> Option<&[u8; N]> {
        self.0.first_chunk()
    }

    pub const fn split_first_chunk<const N: usize>(&self) -> Option<(&[u8; N], &[u8])> {
        self.0.split_first_chunk()
    }

    pub const fn split_last_chunk<const N: usize>(&self) -> Option<(&[u8], &[u8; N])> {
        self.0.split_last_chunk()
    }

    pub const fn last_chunk<const N: usize>(&self) -> Option<&[u8; N]> {
        self.0.last_chunk()
    }

    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        self.0.as_ptr_range()
    }
}

impl<'a> From<&'a [u8]> for &'a Bytes {
    fn from(slice: &'a [u8]) -> Self {
        Bytes::from_slice(slice)
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut Bytes {
    fn from(slice: &'a mut [u8]) -> Self {
        Bytes::from_slice_mut(slice)
    }
}

impl PartialEq<[u8]> for Bytes {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Bytes {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<Bytes> for [u8; N] {
    fn eq(&self, other: &Bytes) -> bool {
        *self == other.0
    }
}

impl PartialEq<Bytes> for [u8] {
    fn eq(&self, other: &Bytes) -> bool {
        *self == other.0
    }
}

impl PartialEq<&[u8]> for Bytes {
    fn eq(&self, other: &&[u8]) -> bool {
        &self.0 == *other
    }
}

impl PartialEq<Bytes> for &[u8] {
    fn eq(&self, other: &Bytes) -> bool {
        **self == other.0
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_from_slice() {
    let data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice(&data);
    assert_eq!(bytes, &data);
}

#[test]
fn test_from_slice_mut() {
    let mut data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice_mut(&mut data);
    bytes[2] = 99;
    assert_eq!(bytes, &[1, 2, 99, 4, 5]);
}

#[test]
fn test_deref() {
    let data = [1, 2, 3];
    let bytes = Bytes::from_slice(&data);
    // Access elements like a slice
    assert_eq!(bytes[0], 1);
    assert_eq!(bytes[1], 2);
    assert_eq!(bytes[2], 3);
}

#[test]
fn test_deref_mut() {
    let mut data = [10, 20, 30, 40];
    let bytes = Bytes::from_slice_mut(&mut data);
    // Mutate elements like a mutable slice
    bytes[1] = 99;
    bytes[3] = 77;
    assert_eq!(bytes, &[10, 99, 30, 77]);
}

#[test]
fn test_len() {
    let data = [5, 6, 7, 8, 9, 10];
    let bytes = Bytes::from_slice(&data);
    assert_eq!(bytes.len(), data.len());
}

#[test]
fn test_is_empty() {
    let empty: &[u8] = &[];
    let bytes = Bytes::from_slice(empty);
    assert!(bytes.is_empty());

    let non_empty = [1];
    let bytes = Bytes::from_slice(&non_empty);
    assert!(!bytes.is_empty());
}

#[test]
fn test_slice() {
    let data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice(&data);
    // Take a slice of Bytes
    let slice = &bytes[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}

#[test]
fn test_mut_slice() {
    let mut data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice_mut(&mut data);
    // Modify a slice of Bytes
    bytes[1..3].copy_from_slice(&[99, 100]);
    assert_eq!(bytes, &[1, 99, 100, 4, 5]);
}

#[test]
fn test_iter() {
    let data = [10, 20, 30, 40];
    let bytes = Bytes::from_slice(&data);
    let mut iter = bytes.iter();

    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert_eq!(iter.next(), Some(&40));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_mut_iter() {
    let mut data = [10, 20, 30, 40];
    let bytes = Bytes::from_slice_mut(&mut data);

    for b in bytes.iter_mut() {
        *b += 1;
    }
    assert_eq!(bytes, &[11, 21, 31, 41]);
}

#[test]
fn test_contains() {
    let data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice(&data);
    assert!(bytes.contains(&3));
    assert!(!bytes.contains(&99));
}

#[test]
fn test_fill() {
    let mut data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice_mut(&mut data);
    bytes.fill(42);
    assert_eq!(bytes, &[42, 42, 42, 42]);
}

#[test]
fn test_cmp() {
    let data1 = [1, 2, 3];
    let data2 = [1, 2, 4];
    let bytes1 = Bytes::from_slice(&data1);
    let bytes2 = Bytes::from_slice(&data2);

    assert!(bytes1 < bytes2);
    assert!(bytes2 > bytes1);
    assert!(bytes1 != bytes2);
}
