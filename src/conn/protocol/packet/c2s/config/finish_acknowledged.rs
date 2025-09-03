use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


#[derive(Debug)]
pub struct C2SConfigFinishAcknowledgedPacket;

impl PacketMeta for C2SConfigFinishAcknowledgedPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x03; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigFinishAcknowledgedPacket {
    type Error = !;

    #[inline(always)]
    fn decode(_ : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
