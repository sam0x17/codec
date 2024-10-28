#![cfg_attr(not(feature = "std"), no_std)]

pub mod bytes;
pub mod bytes_buf;
pub mod bytes_sized;
pub mod io;

use io::*;

pub trait Codec: Sized {
    fn encode(&self, io: &mut impl Write) -> Result<(), WriteError>;
    fn decode(io: &mut impl Read) -> Result<Self, ReadError>;
}

pub fn decode<T: Codec>(io: &mut impl Read) -> Result<T, ReadError> {
    T::decode(io)
}
