use crate::conn::protocol::{
    codec::encode::{
        PacketEncode,
        EncodeBuf
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound,
        s2c::{
            S2CPackets,
            login::S2CLoginPackets
        }
    },
    value::varint::VarInt
};


#[derive(Debug)]
pub struct S2CLoginCompressionPacket {
    pub threshold : u32
}

impl PacketMeta for S2CLoginCompressionPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x03; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginCompressionPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        VarInt::<u32>(self.threshold).encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.threshold).encode(buf);
    } }

}

impl From<S2CLoginCompressionPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CLoginCompressionPacket) -> Self { Self::Login(value.into()) }
}

impl From<S2CLoginCompressionPacket> for S2CLoginPackets<'_> {
    #[inline(always)]
    fn from(value : S2CLoginCompressionPacket) -> Self { Self::Compression(value) }
}
