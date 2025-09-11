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
pub struct C2SPlayTickEndPacket;

impl PacketMeta for C2SPlayTickEndPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x0C; // TODO: Check against current datagen.
}

impl PacketDecode for C2SPlayTickEndPacket {
    type Error = !;

    #[inline(always)]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
