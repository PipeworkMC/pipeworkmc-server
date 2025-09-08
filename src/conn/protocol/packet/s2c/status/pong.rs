use crate::conn::protocol::{
    codec::{
        encode::{
            PacketEncode,
            EncodeBuf
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    },
    packet::s2c::{
        S2CPackets,
        status::S2CStatusPackets
    }
};


#[derive(Debug)]
pub struct S2CStatusPongPacket {
    pub timestamp : u64
}

impl PacketMeta for S2CStatusPongPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CStatusPongPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        PacketEncode::encode_len(&self.timestamp)
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.timestamp.encode(buf);
    } }

}

impl From<S2CStatusPongPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CStatusPongPacket) -> Self { Self::Status(value.into()) }
}

impl From<S2CStatusPongPacket> for S2CStatusPackets<'_> {
    #[inline(always)]
    fn from(value : S2CStatusPongPacket) -> Self { Self::Pong(value) }
}
