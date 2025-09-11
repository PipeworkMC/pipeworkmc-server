use crate::conn::protocol::{
    codec::{
        decode::{
            PacketDecode,
            DecodeBuf,
            IncompleteDecodeError
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    }
};


#[derive(Debug)]
pub struct C2SPlayLoadedPacket;

impl PacketMeta for C2SPlayLoadedPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x2B; // TODO: Check against current datagen.
}

impl PacketDecode for C2SPlayLoadedPacket {
    type Error = !;

    #[inline(always)]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
