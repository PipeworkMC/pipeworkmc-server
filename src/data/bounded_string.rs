use crate::conn::protocol::codec::{
    decode::{
        PacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    encode::{
        PacketEncode,
        EncodeBuf
    }
};
use crate::data::varint::{
    VarInt,
    VarIntDecodeError
};
use core::{
    fmt::{ self, Debug, Display, Formatter },
    ops::Deref,
    ptr
};
use std::str::Utf8Error;
use serde::de::{
    Deserialize as Deser,
    Deserializer as Deserer,
    Error as _
};


#[derive(Clone)]
pub struct BoundedString<const MAX_LEN : usize> {
    data : [u8; MAX_LEN],
    len  : usize
}

impl<const MAX_LEN : usize> BoundedString<MAX_LEN> {
    pub unsafe fn new_unchecked(s : &str) -> Self {
        let mut data = [0u8; MAX_LEN];
        unsafe { ptr::copy_nonoverlapping(s.as_ptr(), data.as_mut_ptr(), s.len()); }
        Self { data, len : s.len() }
    }
}

impl<const MAX_LEN : usize> PacketDecode for BoundedString<MAX_LEN> {
    type Error = BoundedStringDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let length = *VarInt::<u32>::decode(buf).map_err(BoundedStringDecodeError::Length)? as usize;
        if (length > MAX_LEN) {
            return Err(BoundedStringDecodeError::TooLong(TooLong { len : length, max : MAX_LEN }));
        }
        let mut bytes     = [0u8; MAX_LEN];
        let     bytes_buf = &mut bytes[0..length];
        buf.read_buf(bytes_buf)?;
        _ = str::from_utf8(bytes_buf).map_err(BoundedStringDecodeError::Utf8)?;
        Ok(Self { data : bytes, len : length })
    }
}

unsafe impl<const MAX_LEN : usize> PacketEncode for BoundedString<MAX_LEN> {

    #[inline]
    fn encode_len(&self) -> usize {
        let s = unsafe { str::from_utf8_unchecked(&self.data[0..self.len]) };
        s.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        let s = str::from_utf8_unchecked(&self.data[0..self.len]);
        s.encode(buf);
    } }

}


impl<const MAX_LEN : usize> TryFrom<&str> for BoundedString<MAX_LEN> {
    type Error = TooLong;

    fn try_from(value : &str) -> Result<Self, Self::Error> {
        if (value.len() > MAX_LEN) {
            Err(TooLong {
                len : value.len(),
                max : MAX_LEN
            })
        } else {
            let mut bytes = [0u8; MAX_LEN];
            unsafe { ptr::copy_nonoverlapping(
                value.as_ptr(),
                bytes.as_mut_ptr(),
                value.len()
            ); }
            Ok(Self {
                data : bytes,
                len  : value.len()
            })
        }
    }
}


impl<'de, const MAX_LEN : usize> Deser<'de> for BoundedString<MAX_LEN> {
    fn deserialize<D>(deserer : D) -> Result<Self, D::Error>
    where
        D : Deserer<'de>
    {
        match (Self::try_from(<&str>::deserialize(deserer)?)) {
            Ok(s)                     => Ok(s),
            Err(TooLong { len, max }) => Err(D::Error::custom(format!("BoundedString max length of {max} exceeded: {len}")))
        }
    }
}


impl<const MAX_LEN : usize> Deref for BoundedString<MAX_LEN> {
    type Target = str;

    #[inline]
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
    Incomplete(IncompleteDecodeError),
    TooLong(TooLong),
    Utf8(Utf8Error)
}
impl From<IncompleteDecodeError> for BoundedStringDecodeError {
    #[inline(always)]
    fn from(err : IncompleteDecodeError) -> Self { Self::Incomplete(err) }
}
impl Display for BoundedStringDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Length(err)     => write!(f, "length {err}"),
        Self::Incomplete(err) => Display::fmt(err, f),
        Self::TooLong(err)    => write!(f, "length of {} exceeds maximum of {}", err.len, err.max),
        Self::Utf8(_)         => write!(f, "invalid utf8")
    } }
}


#[derive(Debug)]
pub struct TooLong {
    pub len : usize,
    pub max : usize
}
