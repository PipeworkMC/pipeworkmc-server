use crate::conn::protocol::codec::encode::{
    PacketEncode,
    EncodeBuf
};
use crate::data::varint::VarInt;
use core::{
    any::TypeId,
    ops::{ Deref, DerefMut }
};
use std::borrow::Cow;


unsafe impl<T> PacketEncode for [T]
where
    T : PacketEncode + 'static
{

    #[inline]
    fn encode_len(&self) -> usize {
        let prefix_len = VarInt::<u32>(self.len() as u32).encode_len();
        if (TypeId::of::<T>() == TypeId::of::<u8>()) {
            prefix_len + self.len()
        } else {
            prefix_len + self.iter().map(|item| item.encode_len()).sum::<usize>()
        }
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
    T   : PacketEncode + 'static,
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
    T : PacketEncode + 'static
{

    #[inline(always)]
    fn encode_len(&self) -> usize { <[T]>::encode_len(self) }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        <[T]>::encode(self, buf)
    } }

}


#[derive(Clone, Debug)]
pub struct UnprefixedVec<T>(pub Vec<T>);

impl<T> From<Vec<T>> for UnprefixedVec<T> {
    #[inline(always)]
    fn from(value : Vec<T>) -> Self { Self(value) }
}

impl<T> Deref for UnprefixedVec<T> {
    type Target = Vec<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for UnprefixedVec<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

unsafe impl<T> PacketEncode for UnprefixedVec<T>
where
    T : PacketEncode + 'static
{

    #[inline]
    fn encode_len(&self) -> usize {
        if (TypeId::of::<T>() == TypeId::of::<u8>()) {
            self.len()
        } else {
            self.iter().map(|item| item.encode_len()).sum()
        }
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        for item in &**self {
            item.encode(buf);
        }
    } }

}
