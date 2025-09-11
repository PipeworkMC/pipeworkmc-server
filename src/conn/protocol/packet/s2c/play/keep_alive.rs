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
        play::S2CPlayPackets
    }
};


#[derive(Debug)]
pub struct S2CPlayKeepAlivePacket {
    pub id : u64
}

impl PacketMeta for S2CPlayKeepAlivePacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x26; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayKeepAlivePacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.id.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.id.encode(buf);
    } }

}

impl From<S2CPlayKeepAlivePacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayKeepAlivePacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayKeepAlivePacket> for S2CPlayPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayKeepAlivePacket) -> Self { Self::KeepAlive(value) }
}
