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
use std::string::FromUtf8Error;


impl PacketDecode for String {
    type Error = StringDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let length = *VarInt::<u32>::decode(buf).map_err(StringDecodeError::Length)? as usize;
        let bytes  = buf.read_vec(length)?;
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
