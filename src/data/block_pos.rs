use crate::conn::protocol::codec::encode::{
    PacketEncode,
    EncodeBuf
};
use crate::data::ident::Ident;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockPos {
    pub x : i32,
    pub y : i16,
    pub z : i32
}

impl BlockPos {
    pub const ZERO : Self = Self { x : 0, y : 0, z : 0 };
}

unsafe impl PacketEncode for BlockPos {

    #[inline(always)]
    fn encode_len(&self) -> usize { 8 }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        ((((self.x as u64) & 0x3FFFFFF) << 38) | (((self.z as u64) & 0x3FFFFFF) << 12) | ((self.y as u64) & 0xFFF))
            .encode(buf);
    } }

}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DimBlockPos {
    pub dim : Ident,
    pub pos : BlockPos
}

unsafe impl PacketEncode for DimBlockPos {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.dim.encode_len()
        + self.pos.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.dim.encode(buf);
        self.pos.encode(buf);
    } }

}
