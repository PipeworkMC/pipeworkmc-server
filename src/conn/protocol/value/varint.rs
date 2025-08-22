use crate::conn::protocol::codec::decode::{
    PacketDecode,
    DecodeBuf,
    IncompleteDecodeError
};
use core::ops::{ Add, BitOr, Deref, Shl };


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VarInt<T>(T)
where
    T : VarIntType;


impl<T> Deref for VarInt<T>
where
    T : VarIntType
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub const SEGMENT_BITS : u8 = 0b01111111;
pub const CONTINUE_BIT : u8 = 0b10000000;


pub unsafe trait PureVarIntType
where Self
        : VarIntType
        + From<u8>
        + Add<Self, Output = Self>
        + BitOr<Self, Output = Self>
        + Shl<Self, Output = Self>
        + PartialOrd<Self>
        + Clone + Copy
        + Sized
{
    const ZERO      : Self;
    const SEVEN     : Self;
    const MAX_SHIFT : Self;
    fn le_to_be(value : Self) -> Self;

    fn decode<I>(mut iter : I) -> Result<(Self, usize,), VarIntDecodeError>
    where
        I : Iterator<Item = u8>
    {
        let mut value    = Self::ZERO;
        let mut shift    = Self::ZERO;
        let mut consumed = 0;
        loop {
            let b = iter.next().ok_or(VarIntDecodeError::Incomplete)?;
            consumed += 1;
            value = value | ((Self::from(b & SEGMENT_BITS)) << shift);
            if ((b & CONTINUE_BIT) == 0) { break; }
            shift = shift + Self::SEVEN;
            if (shift > Self::MAX_SHIFT) { return Err(VarIntDecodeError::TooLong); }
        }
        Ok((Self::le_to_be(value), consumed,))
    }

}

unsafe impl PureVarIntType for i32 {
    const ZERO      : Self = 0;
    const SEVEN     : Self = 7;
    const MAX_SHIFT : Self = Self::BITS as Self;
    fn le_to_be(value : Self) -> Self { Self::from_le(value) }
}

unsafe impl PureVarIntType for i64 {
    const ZERO      : Self = 0;
    const SEVEN     : Self = 7;
    const MAX_SHIFT : Self = Self::BITS as Self;
    fn le_to_be(value : Self) -> Self { Self::from_le(value) }
}


pub trait VarIntType
where
    Self : Sized
{
    const MAX_BYTES : usize = size_of::<Self>() + 1;

    fn decode<I>(iter : I) -> Result<(VarInt<Self>, usize,), VarIntDecodeError>
    where
        I : Iterator<Item = u8>;

}

impl<T : PureVarIntType> VarIntType for T {
    #[inline(always)]
    fn decode<I>(iter : I) -> Result<(VarInt<Self>, usize,), VarIntDecodeError>
    where
        I : Iterator<Item = u8>
    { <Self as PureVarIntType>::decode(iter).map(|(value, consumed,)|
        (VarInt(value), consumed,)
    ) }
}

impl VarIntType for u32 {
    #[inline(always)]
    fn decode<I>(iter : I) -> Result<(VarInt<Self>, usize,), VarIntDecodeError>
    where
        I : Iterator<Item = u8>
    { <i32 as PureVarIntType>::decode(iter).map(|(value, consumed,)|
        (VarInt(value.cast_unsigned()), consumed,)
    ) }
}

impl VarIntType for u64 {
    #[inline(always)]
    fn decode<I>(iter : I) -> Result<(VarInt<Self>, usize,), VarIntDecodeError>
    where
        I : Iterator<Item = u8>
    { <i64 as PureVarIntType>::decode(iter).map(|(value, consumed,)|
        (VarInt(value.cast_unsigned()), consumed,)
    ) }
}


impl<T> PacketDecode for VarInt<T>
where
    T : VarIntType
{
    type Error = VarIntDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let (value, consumed,) = T::decode(buf.iter())?;
        buf.skip(consumed)?;
        Ok(value)
    }
}

#[derive(Debug)]
pub enum VarIntDecodeError {
    Incomplete,
    TooLong
}

impl From<IncompleteDecodeError> for VarIntDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteDecodeError) -> Self {
        Self::Incomplete
    }
}
