use crate::conn::protocol::{
    codec::{
        decode::{
            PacketDecode,
            DecodeBuf
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    }
};


#[derive(Debug)]
pub struct C2SLoginFinishAcknowledgedPacket;

impl PacketMeta for C2SLoginFinishAcknowledgedPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x03; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginFinishAcknowledgedPacket {
    type Error = !;

    #[inline(always)]
    fn decode(_ : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
