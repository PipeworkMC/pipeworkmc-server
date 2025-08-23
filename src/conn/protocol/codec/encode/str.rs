use crate::conn::protocol::{
    codec::encode::{
        PacketEncode,
        EncodeBuf
    },
    value::varint::VarInt
};


unsafe impl PacketEncode for &str {

    #[inline]
    fn encode_len(&self) -> usize {
        VarInt::<u32>(self.len() as u32).encode_len()
        + self.as_bytes().len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        buf.encode_write(&VarInt::<u32>(self.len() as u32));
        buf.write_slice(self.as_bytes());
    } }

}


unsafe impl PacketEncode for String {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.as_str().encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.as_str().encode(buf)
    } }

}
