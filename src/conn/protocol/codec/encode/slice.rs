use crate::conn::protocol::{
    codec::encode::{
        PacketEncode,
        EncodeBuf
    },
    value::varint::VarInt
};
use std::borrow::Cow;


unsafe impl<T> PacketEncode for [T]
where
    T : PacketEncode
{

    fn encode_len(&self) -> usize {
        let mut len = VarInt::<u32>(self.len() as u32).encode_len();
        for item in self {
            len += item.encode_len();
        }
        len
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.len() as u32).encode(buf);
        for item in self {
            item.encode(buf);
        }
    } }

}


unsafe impl<T> PacketEncode for Cow<'_, [T]>
where
    T   : PacketEncode,
    [T] : ToOwned
{

    #[inline(always)]
    fn encode_len(&self) -> usize { <[T]>::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        <[T]>::encode(self, buf)
    } }

}


unsafe impl<T> PacketEncode for Vec<T>
where
    T : PacketEncode
{

    #[inline(always)]
    fn encode_len(&self) -> usize { <[T]>::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        <[T]>::encode(self, buf)
    } }

}
