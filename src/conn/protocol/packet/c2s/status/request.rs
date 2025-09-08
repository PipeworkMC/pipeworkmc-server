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
pub struct C2SStatusRequestPacket;

impl PacketMeta for C2SStatusRequestPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for C2SStatusRequestPacket {
    type Error = !;

    #[inline(always)]
    fn decode(_ : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
