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
        config::S2CConfigPackets
    }
};


#[derive(Debug)]
pub struct S2CConfigKeepAlivePacket {
    pub id : u64
}

impl PacketMeta for S2CConfigKeepAlivePacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x04; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigKeepAlivePacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.id.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.id.encode(buf);
    } }

}

impl From<S2CConfigKeepAlivePacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigKeepAlivePacket) -> Self { Self::Config(value.into()) }
}

impl From<S2CConfigKeepAlivePacket> for S2CConfigPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigKeepAlivePacket) -> Self { Self::KeepAlive(value) }
}
