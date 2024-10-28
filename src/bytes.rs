use core::ops::{Deref, DerefMut, Range};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bytes([u8]);

impl Bytes {
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Create a new `Bytes` from a slice of bytes
    #[inline]
    pub const fn from_slice(slice: &[u8]) -> &Self {
        // Safety: `Bytes` is a transparent wrapper around `[u8]`
        unsafe { &*(slice as *const [u8] as *const Bytes) }
    }

    /// Create a new `Bytes` from a mutable slice of bytes
    #[inline]
    pub fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
        // Safety: `Bytes` is a transparent wrapper around `[u8]`
        unsafe { &mut *(slice as *mut [u8] as *mut Bytes) }
    }

    #[inline]
    pub const fn split_at(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = self.0.split_at(mid);
        (Bytes::from_slice(left), Bytes::from_slice(right))
    }

    #[inline]
    pub const fn split_at_checked(&self, mid: usize) -> Option<(&Self, &Self)> {
        match self.0.split_at_checked(mid) {
            Some((left, right)) => Some((Bytes::from_slice(left), Bytes::from_slice(right))),
            None => None,
        }
    }

    #[inline]
    pub const fn split_at_unchecked(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = unsafe { self.0.split_at_unchecked(mid) };
        (Bytes::from_slice(left), Bytes::from_slice(right))
    }

    #[inline]
    pub const fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    #[inline]
    pub const fn trim_ascii_start(&self) -> &Self {
        Bytes::from_slice(self.0.trim_ascii_start())
    }

    #[inline]
    pub const fn trim_ascii_end(&self) -> &Self {
        Bytes::from_slice(self.0.trim_ascii_end())
    }

    #[inline]
    pub const fn trim_ascii(&self) -> &Self {
        Bytes::from_slice(self.0.trim_ascii())
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub const fn first(&self) -> Option<&u8> {
        self.0.first()
    }

    #[inline]
    pub const fn last(&self) -> Option<&u8> {
        self.0.last()
    }

    #[inline]
    pub const fn split_first(&self) -> Option<(&u8, &Self)> {
        match self.0.split_first() {
            Some((first, rest)) => Some((first, Bytes::from_slice(rest))),
            None => None,
        }
    }

    #[inline]
    pub const fn split_last(&self) -> Option<(&u8, &Self)> {
        match self.0.split_last() {
            Some((last, rest)) => Some((last, Bytes::from_slice(rest))),
            None => None,
        }
    }

    #[inline]
    pub const fn first_chunk<const N: usize>(&self) -> Option<&[u8; N]> {
        self.0.first_chunk()
    }

    #[inline]
    pub const fn split_first_chunk<const N: usize>(&self) -> Option<(&[u8; N], &[u8])> {
        self.0.split_first_chunk()
    }

    #[inline]
    pub const fn split_last_chunk<const N: usize>(&self) -> Option<(&[u8], &[u8; N])> {
        self.0.split_last_chunk()
    }

    #[inline]
    pub const fn last_chunk<const N: usize>(&self) -> Option<&[u8; N]> {
        self.0.last_chunk()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        self.0.as_ptr_range()
    }
}

impl Default for &Bytes {
    fn default() -> Self {
        Bytes::from_slice(&[])
    }
}

impl Default for &mut Bytes {
    fn default() -> Self {
        Bytes::from_slice_mut(&mut [])
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

impl<'a, const N: usize> From<&'a [u8; N]> for &'a Bytes {
    fn from(data: &'a [u8; N]) -> Self {
        Bytes::from_slice(data)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for &'a mut Bytes {
    fn from(data: &'a mut [u8; N]) -> Self {
        Bytes::from_slice_mut(data)
    }
}

impl<'a> From<&'a Bytes> for &'a [u8] {
    fn from(bytes: &'a Bytes) -> Self {
        &bytes.0
    }
}

impl<'a> From<&'a mut Bytes> for &'a mut [u8] {
    fn from(bytes: &'a mut Bytes) -> Self {
        &mut bytes.0
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

#[test]
fn test_split_at() {
    let data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice(&data);
    let (left, right) = bytes.split_at(3);
    assert_eq!(left, &[1, 2, 3]);
    assert_eq!(right, &[4, 5]);
}

#[test]
fn test_split_at_bounds() {
    let data = [10, 20, 30, 40];
    let bytes = Bytes::from_slice(&data);

    // Split at the beginning
    let (left, right) = bytes.split_at(0);
    assert!(left.is_empty());
    assert_eq!(right, &[10, 20, 30, 40]);

    // Split at the end
    let (left, right) = bytes.split_at(4);
    assert_eq!(left, &[10, 20, 30, 40]);
    assert!(right.is_empty());
}

#[test]
fn test_trim_ascii_start() {
    let data = b"   Hello";
    let bytes = Bytes::from_slice(data);
    let trimmed = bytes.trim_ascii_start();
    assert_eq!(trimmed, b"Hello");
}

#[test]
fn test_trim_ascii_end() {
    let data = b"Hello   ";
    let bytes = Bytes::from_slice(data);
    let trimmed = bytes.trim_ascii_end();
    assert_eq!(trimmed, b"Hello");
}

#[test]
fn test_trim_ascii() {
    let data = b"   Hello   ";
    let bytes = Bytes::from_slice(data);
    let trimmed = bytes.trim_ascii();
    assert_eq!(trimmed, b"Hello");
}

#[test]
fn test_is_ascii() {
    let ascii_data = b"Hello, World!";
    let non_ascii_data = &[0xFF, 0x80, 0x7F, 0x40];

    let ascii_bytes = Bytes::from_slice(ascii_data);
    let non_ascii_bytes = Bytes::from_slice(non_ascii_data);

    assert!(ascii_bytes.is_ascii());
    assert!(!non_ascii_bytes.is_ascii());
}

#[test]
fn test_first_and_last() {
    let data = [1, 2, 3, 4, 5];
    let bytes = Bytes::from_slice(&data);

    assert_eq!(bytes.first(), Some(&1));
    assert_eq!(bytes.last(), Some(&5));

    let empty: &[u8] = &[];
    let empty_bytes = Bytes::from_slice(empty);
    assert_eq!(empty_bytes.first(), None);
    assert_eq!(empty_bytes.last(), None);
}

#[test]
fn test_split_first() {
    let data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice(&data);

    if let Some((first, rest)) = bytes.split_first() {
        assert_eq!(first, &1);
        assert_eq!(rest, &[2, 3, 4]);
    } else {
        panic!("Expected Some((first, rest))");
    }

    let empty: &[u8] = &[];
    let empty_bytes = Bytes::from_slice(empty);
    assert_eq!(empty_bytes.split_first(), None);
}

#[test]
fn test_split_last() {
    let data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice(&data);

    if let Some((last, rest)) = bytes.split_last() {
        assert_eq!(last, &4);
        assert_eq!(rest, &[1, 2, 3]);
    } else {
        panic!("Expected Some((last, rest))");
    }

    let empty: &[u8] = &[];
    let empty_bytes = Bytes::from_slice(empty);
    assert_eq!(empty_bytes.split_last(), None);
}

#[test]
fn test_first_chunk() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    let bytes = Bytes::from_slice(&data);

    let chunk = bytes.first_chunk::<3>();
    assert_eq!(chunk, Some(&[1, 2, 3]));

    let chunk = bytes.first_chunk::<8>();
    assert_eq!(chunk, None);
}

#[test]
fn test_last_chunk() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    let bytes = Bytes::from_slice(&data);

    let chunk = bytes.last_chunk::<3>();
    assert_eq!(chunk, Some(&[5, 6, 7]));

    let chunk = bytes.last_chunk::<8>();
    assert_eq!(chunk, None);
}

#[test]
fn test_split_first_chunk() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    let bytes = Bytes::from_slice(&data);

    if let Some((chunk, rest)) = bytes.split_first_chunk::<3>() {
        assert_eq!(chunk, &[1, 2, 3]);
        assert_eq!(rest, &[4, 5, 6, 7]);
    } else {
        panic!("Expected Some((chunk, rest))");
    }

    let bytes = Bytes::from_slice(&[1, 2]);
    assert_eq!(bytes.split_first_chunk::<3>(), None);
}

#[test]
fn test_split_last_chunk() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    let bytes = Bytes::from_slice(&data);

    if let Some((rest, chunk)) = bytes.split_last_chunk::<3>() {
        assert_eq!(rest, &[1, 2, 3, 4]);
        assert_eq!(chunk, &[5, 6, 7]);
    } else {
        panic!("Expected Some((rest, chunk))");
    }

    let bytes = Bytes::from_slice(&[1, 2]);
    assert_eq!(bytes.split_last_chunk::<3>(), None);
}

#[test]
fn test_as_ptr() {
    let data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice(&data);
    assert_eq!(bytes.as_ptr(), data.as_ptr());
}

#[test]
fn test_as_ptr_range() {
    let data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice(&data);
    assert_eq!(bytes.as_ptr_range(), data.as_ptr_range());
}

#[test]
fn test_empty_bytes() {
    let empty: &[u8] = &[];
    let bytes = Bytes::from_slice(empty);
    assert_eq!(bytes.len(), 0);
    assert!(bytes.is_empty());
    assert_eq!(bytes.first(), None);
    assert_eq!(bytes.last(), None);
    assert_eq!(bytes.split_first(), None);
    assert_eq!(bytes.split_last(), None);
}

#[test]
fn test_partial_eq_with_slice() {
    let data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice(&data);
    assert_eq!(bytes, &[1, 2, 3, 4]);
    assert_ne!(bytes, &[1, 2, 3]);
}

#[test]
fn test_partial_eq_with_array() {
    let data = [1, 2, 3, 4];
    let bytes = Bytes::from_slice(&data);
    assert_eq!(bytes, &[1, 2, 3, 4]);
    assert_ne!(bytes, &[1, 2, 3]);
}
