use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    value::varint::{
        VarInt,
        VarIntDecodeError
    }
};
use core::{
    fmt::{ self, Debug, Display, Formatter },
    ops::Deref
};
use std::str::Utf8Error;


pub struct BoundedString<const MAX_LEN : usize> {
    data : [u8; MAX_LEN],
    len  : usize
}

impl<const MAX_LEN : usize> PacketDecode for BoundedString<MAX_LEN> {
    type Error = BoundedStringDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let length = *buf.read_decode::<VarInt<u32>>().map_err(BoundedStringDecodeError::Length)? as usize;
        if (length > MAX_LEN) {
            return Err(BoundedStringDecodeError::TooLong { len : length, max : MAX_LEN });
        }
        let mut bytes     = [0u8; MAX_LEN];
        let     bytes_buf = &mut bytes[0..length];
        buf.read_buf(bytes_buf)?;
        _ = str::from_utf8(bytes_buf).map_err(BoundedStringDecodeError::Utf8)?;
        Ok(Self { data : bytes, len : length })
    }
}


impl<const MAX_LEN : usize> Deref for BoundedString<MAX_LEN> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { str::from_utf8_unchecked(&self.data[0..self.len]) }
    }
}

impl<const MAX_LEN : usize> Debug for BoundedString<MAX_LEN> {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self as &str, f)
    }
}
impl<const MAX_LEN : usize> Display for BoundedString<MAX_LEN> {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self as &str, f)
    }
}


#[derive(Debug)]
pub enum BoundedStringDecodeError {
    Length(VarIntDecodeError),
    Incomplete,
    TooLong {
        len : usize,
        max : usize
    },
    Utf8(Utf8Error)
}
impl From<IncompleteDecodeError> for BoundedStringDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteDecodeError) -> Self { Self::Incomplete }
}
