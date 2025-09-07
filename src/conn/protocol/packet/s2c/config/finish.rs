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
            config::S2CConfigPackets
        }
    }
};


#[derive(Debug)]
pub struct S2CConfigFinishPacket;


impl PacketMeta for S2CConfigFinishPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x03; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigFinishPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize { 0 }

    #[inline(always)]
    unsafe fn encode(&self, _ : &mut EncodeBuf) { }

}

impl From<S2CConfigFinishPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigFinishPacket) -> Self { Self::Config(value.into()) }
}

impl From<S2CConfigFinishPacket> for S2CConfigPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigFinishPacket) -> Self { Self::Finish(value) }
}
