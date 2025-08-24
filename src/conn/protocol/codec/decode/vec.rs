use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf
    },
    value::varint::{
        VarInt,
        VarIntDecodeError
    }
};


impl<T> PacketDecode for Vec<T>
where
    T : PacketDecode
{
    type Error = VecDecodeError<T::Error>;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        let     length = *VarInt::<u32>::decode(buf).map_err(VecDecodeError::Length)? as usize;
        let mut vec    = Vec::with_capacity(length);
        for i in 0..length {
            vec.push(T::decode(buf).map_err(|err| VecDecodeError::Entry { index : i, err })?);
        }
        Ok(vec)
    }
}


#[derive(Debug)]
pub enum VecDecodeError<E> {
    Length(VarIntDecodeError),
    Entry {
        index : usize,
        err   : E
    }
}
