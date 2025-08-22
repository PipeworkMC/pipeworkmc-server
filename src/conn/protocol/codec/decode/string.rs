use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        IncompleteDecodeError
    },
    value::varint::{
        VarInt,
        VarIntDecodeError
    }
};
use std::string::FromUtf8Error;


impl PacketDecode for String {
    type Error = StringDecodeError;

    fn decode(buf : &mut super::DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let length = buf.read_decode::<VarInt<u32>>().map_err(StringDecodeError::Length)?;
        let bytes  = buf.read_n(*length as usize)?;
        let string = String::from_utf8(bytes).map_err(StringDecodeError::Utf8)?;
        Ok(string)
    }
}


#[derive(Debug)]
pub enum StringDecodeError {
    Length(VarIntDecodeError),
    Incomplete,
    Utf8(FromUtf8Error)
}
impl From<IncompleteDecodeError> for StringDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteDecodeError) -> Self { Self::Incomplete }
}
