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
pub struct C2SLoginFinishPacket;

impl PacketMeta for C2SLoginFinishPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x03; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginFinishPacket {
    type Error = !;

    #[inline(always)]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
